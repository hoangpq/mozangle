#! /usr/bin/env python3
assert __name__ == '__main__'

'''
To update ANGLE in Gecko, use Windows with git-bash, and setup depot_tools, python2, and
python3. Because depot_tools expects `python` to be `python2` (shame!), python2 must come
before python3 in your path.

Upstream: https://chromium.googlesource.com/angle/angle

Our repo: https://github.com/mozilla/angle
It has branches like 'firefox-60' which is the branch we use for pulling into
Gecko with this script.

This script leaves a record of the merge-base and cherry-picks that we pull into
Gecko. (gfx/angle/cherries.log)

ANGLE<->Chrome version mappings are here: https://omahaproxy.appspot.com/
An easy choice is to grab Chrome's Beta's ANGLE branch.

## Usage

Prepare your env:

~~~
export PATH="$PATH:/path/to/depot_tools"
~~~

If this is a new repo, don't forget:

~~~
# In the angle repo:
./scripts/bootstrap.py
gclient sync
~~~

Update: (in the angle repo)

~~~
# In the angle repo:
/path/to/gecko/gfx/angle/update-angle.py origin/chromium/XXXX
git push moz # Push the firefox-XX branch to github.com/mozilla/angle
~~~~

'''

import json
import os
import pathlib
import re
import shutil
import subprocess
import sys
from typing import * # mypy annotations

REPO_DIR = pathlib.Path.cwd()
GECKO_ANGLE_DIR = pathlib.Path(__file__).parent

OUT_DIR = pathlib.Path('out')

COMMON_HEADER = [
    '# Generated by update-angle.py',
    '',
    "include('../../moz.build.common')",
]

ROOTS = ['//:translator', '//:libEGL', '//:libGLESv2']

CHECK_ONLY = False
args = sys.argv[1:]
while True:
    arg = args.pop(0)
    if arg == '--check':
        CHECK_ONLY = True
        continue
    args.insert(0, arg)
    break

GN_ENV = dict(os.environ)
GN_ENV['DEPOT_TOOLS_WIN_TOOLCHAIN'] = '0'

(GIT_REMOTE, ) = args # Not always 'origin'!

# ------------------------------------------------------------------------------

def run_checked(*args, **kwargs):
    print(' ', args)
    sys.stdout.flush()
    return subprocess.run(args, check=True, **kwargs)


def sorted_items(x):
    for k in sorted(x.keys()):
        yield (k, x[k])


def collapse_dotdots(path):
    split = path.split('/')

    ret = []
    for x in split:
        if x == '..' and ret:
            ret.pop()
            continue
        ret.append(x)
        continue

    return '/'.join(ret)


def dag_traverse(root_keys: Sequence[str], pre_recurse_func: Callable[[str], list]):
    visited_keys: Set[str] = set()

    def recurse(key):
        if key in visited_keys:
            return
        visited_keys.add(key)

        t = pre_recurse_func(key)
        try:
            (next_keys, post_recurse_func) = t
        except ValueError:
            (next_keys,) = t
            post_recurse_func = None

        for x in next_keys:
            recurse(x)

        if post_recurse_func:
            post_recurse_func(key)
        return

    for x in root_keys:
        recurse(x)
    return

# ------------------------------------------------------------------------------

print('Importing graph')

#shutil.rmtree(str(OUT_DIR), True)
OUT_DIR.mkdir(exist_ok=True)

GN_ARGS = b'''
# Build arguments go here.
# See "gn args <out_dir> --list" for available build arguments.
is_clang = false
angle_enable_gl = false
angle_enable_gl_null = false
angle_enable_null = false
angle_enable_vulkan = false
'''[1:]
args_gn_path = OUT_DIR / 'args.gn'
args_gn_path.write_bytes(GN_ARGS)

try:
    run_checked('gn', 'gen', str(OUT_DIR), shell=True, env=GN_ENV)
except subprocess.CalledProcessError:
    sys.stderr.buffer.write(b'`gn` failed. Is depot_tools in your PATH?\n')
    exit(1)

p = run_checked('gn', 'desc', '--format=json', str(OUT_DIR), '*', stdout=subprocess.PIPE,
                shell=True, env=GN_ENV)

# -

print('\nProcessing graph')
descs = json.loads(p.stdout.decode())

# -
# HACKHACKHACK: Inject linux/mac sources instead of trying to merge graphs of different
# platforms.
descs['//:angle_common']['sources'] += [
    '//src/common/system_utils_linux.cpp',
    '//src/common/system_utils_mac.cpp',
    '//src/common/system_utils_posix.cpp',
]

# Ready to traverse
# ------------------------------------------------------------------------------

LIBRARY_TYPES = ('shared_library', 'static_library')

def flattened_target(target_name: str, descs: dict, stop_at_lib: bool =True) -> dict:
    flattened = dict(descs[target_name])

    EXPECTED_TYPES = LIBRARY_TYPES + ('source_set', 'group', 'action')

    def pre(k):
        dep = descs[k]

        dep_type = dep['type']
        deps = dep['deps']
        if stop_at_lib and dep_type in LIBRARY_TYPES:
            return ((),)

        if dep_type == 'copy':
            assert not deps, (target_name, dep['deps'])
        else:
            assert dep_type in EXPECTED_TYPES, (k, dep_type)
            for (k,v) in dep.items():
                if type(v) in (list, tuple):
                    flattened[k] = flattened.get(k, []) + v
                else:
                    #flattened.setdefault(k, v)
                    pass
        return (deps,)

    dag_traverse(descs[target_name]['deps'], pre)
    return flattened

# ------------------------------------------------------------------------------
# Check that includes are valid. (gn's version of this check doesn't seem to work!)

INCLUDE_REGEX = re.compile(b'(?:^|\\n) *# *include +([<"])([^>"]+)[>"]')
assert INCLUDE_REGEX.match(b'#include "foo"')
assert INCLUDE_REGEX.match(b'\n#include "foo"')

IGNORED_INCLUDES = {
    b'compiler/translator/TranslatorVulkan.h',
    b'libANGLE/renderer/d3d/d3d11/winrt/NativeWindow11WinRT.h',
    b'libANGLE/renderer/gl/glx/DisplayGLX.h',
    b'libANGLE/renderer/gl/cgl/DisplayCGL.h',
    b'libANGLE/renderer/gl/egl/ozone/DisplayOzone.h',
    b'libANGLE/renderer/gl/egl/android/DisplayAndroid.h',
    b'libANGLE/renderer/gl/wgl/DisplayWGL.h',
    b'libANGLE/renderer/null/DisplayNULL.h',
    b'libANGLE/renderer/vulkan/android/DisplayVkAndroid.h',
    b'libANGLE/renderer/vulkan/fuchsia/DisplayVkFuchsia.h',
    b'libANGLE/renderer/vulkan/win32/DisplayVkWin32.h',
    b'libANGLE/renderer/vulkan/xcb/DisplayVkXcb.h',
    b'kernel/image.h',
}

IGNORED_INCLUDE_PREFIXES = {
    b'android',
    b'Carbon',
    b'CoreFoundation',
    b'CoreServices',
    b'IOSurface',
    b'mach',
    b'mach-o',
    b'OpenGL',
    b'pci',
    b'sys',
    b'wrl',
    b'X11',
}

def has_all_includes(target_name: str, descs: dict) -> bool:
    flat = flattened_target(target_name, descs, stop_at_lib=False)
    acceptable_sources = flat.get('sources', []) + flat.get('outputs', [])
    acceptable_sources = (x.rsplit('/', 1)[-1].encode() for x in acceptable_sources)
    acceptable_sources = set(acceptable_sources)

    ret = True
    desc = descs[target_name]
    for cur_file in desc.get('sources', []):
        assert cur_file.startswith('/'), cur_file
        if not cur_file.startswith('//'):
            continue
        cur_file = pathlib.Path(cur_file[2:])
        text = cur_file.read_bytes()
        for m in INCLUDE_REGEX.finditer(text):
            if m.group(1) == b'<':
                continue
            include = m.group(2)
            if include in IGNORED_INCLUDES:
                continue
            try:
                (prefix, _) = include.split(b'/', 1)
                if prefix in IGNORED_INCLUDE_PREFIXES:
                    continue
            except ValueError:
                pass

            include_file = include.rsplit(b'/', 1)[-1]
            if include_file not in acceptable_sources:
                #print('  acceptable_sources:')
                #for x in sorted(acceptable_sources):
                #    print('   ', x)
                print('Warning in {}: {}: Invalid include: {}'.format(target_name, cur_file, include))
                ret = False
            #print('Looks valid:', m.group())
            continue

    return ret

# -
# Gather real targets:

def gather_libraries(roots: Sequence[str], descs: dict) -> Set[str]:
    libraries = set()
    def fn(target_name):
        cur = descs[target_name]
        print('  ' + cur['type'], target_name)
        assert has_all_includes(target_name, descs), target_name

        if cur['type'] in ('shared_library', 'static_library'):
            libraries.add(target_name)
        return (cur['deps'], )

    dag_traverse(roots, fn)
    return libraries

# -

libraries = gather_libraries(ROOTS, descs)
print(f'\n{len(libraries)} libraries:')
for k in libraries:
    print(' ', k)

if CHECK_ONLY:
    print('\n--check complete.')
    exit(0)

# ------------------------------------------------------------------------------
# Output to moz.builds

import vendor_from_git

print('')
vendor_from_git.record_cherry_picks(GECKO_ANGLE_DIR, GIT_REMOTE)

# --

def sortedi(x):
    return sorted(x, key=str.lower)

def append_arr(dest, name, vals, indent=0):
    if not vals:
        return

    dest.append('{}{} += ['.format(' '*4*indent, name))
    for x in sortedi(vals):
        dest.append("{}'{}',".format(' '*4*(indent+1), x))
    dest.append('{}]'.format(' '*4*indent))
    dest.append('')
    return

REGISTERED_DEFINES = {
    'ANGLE_EGL_LIBRARY_NAME': False,
    'ANGLE_ENABLE_D3D11': True,
    'ANGLE_ENABLE_D3D9': True,
    'ANGLE_ENABLE_DEBUG_ANNOTATIONS': True,
    'ANGLE_ENABLE_NULL': False,
    'ANGLE_ENABLE_OPENGL': False,
    'ANGLE_ENABLE_OPENGL_NULL': False,
    'ANGLE_ENABLE_ESSL': True,
    'ANGLE_ENABLE_GLSL': True,
    'ANGLE_ENABLE_HLSL': True,
    'ANGLE_GENERATE_SHADER_DEBUG_INFO': True,
    'ANGLE_GLESV2_LIBRARY_NAME': True,
    'ANGLE_IS_64_BIT_CPU': False,
    'ANGLE_PRELOADED_D3DCOMPILER_MODULE_NAMES': False,
    'ANGLE_USE_EGL_LOADER': True,
    'CERT_CHAIN_PARA_HAS_EXTRA_FIELDS': False,
    'CHROMIUM_BUILD': False,
    'COMPONENT_BUILD': False,
    'DYNAMIC_ANNOTATIONS_ENABLED': True,
    'EGL_EGL_PROTOTYPES': True,
    'EGL_EGLEXT_PROTOTYPES': True,
    'EGLAPI': True,
    'FIELDTRIAL_TESTING_ENABLED': False,
    'FULL_SAFE_BROWSING': False,
    'GL_API': True,
    'GL_APICALL': True,
    'GL_GLES_PROTOTYPES': True,
    'GL_GLEXT_PROTOTYPES': True,
    'GPU_INFO_USE_SETUPAPI': True,
    'LIBANGLE_IMPLEMENTATION': True,
    'LIBEGL_IMPLEMENTATION': True,
    'LIBGLESV2_IMPLEMENTATION': True,
    'NOMINMAX': True,
    'NO_TCMALLOC': False,

    # Else: gfx/angle/checkout/src/libANGLE/renderer/d3d/d3d11/win32/NativeWindow11Win32.cpp(89): error C2787: 'IDCompositionDevice': no GUID has been associated with this object
    'NTDDI_VERSION': True,

    'PSAPI_VERSION': False,
    'SAFE_BROWSING_CSD': False,
    'SAFE_BROWSING_DB_LOCAL': False,
    'UNICODE': True,
    'USE_AURA': False,
    'V8_DEPRECATION_WARNINGS': False,
    'WIN32': False,
    'WIN32_LEAN_AND_MEAN': False,
    'WINAPI_FAMILY': False,

    'WINVER': True,
    # Otherwise:
    # gfx/angle/targets/libANGLE
    # In file included from c:/dev/mozilla/gecko4/gfx/angle/checkout/src/libANGLE/renderer/d3d/d3d11/converged/CompositorNativeWindow11.cpp:10:
    # In file included from c:/dev/mozilla/gecko4/gfx/angle/checkout/src\libANGLE/renderer/d3d/d3d11/converged/CompositorNativeWindow11.h:17:
    # C:\Program Files (x86)\Windows Kits\10\include\10.0.17763.0\winrt\Windows.ui.composition.interop.h(103,20): error: unknown type name 'POINTER_INFO'
    #         _In_ const POINTER_INFO& pointerInfo
    #                    ^

    'WTF_USE_DYNAMIC_ANNOTATIONS': False,
    '_ATL_NO_OPENGL': True,
    '_CRT_RAND_S': True,
    '_CRT_SECURE_NO_DEPRECATE': True,
    '_DEBUG': False,
    '_HAS_EXCEPTIONS': True,
    '_HAS_ITERATOR_DEBUGGING': False,
    '_SCL_SECURE_NO_DEPRECATE': True,
    '_SECURE_ATL': True,
    '_UNICODE': True,
    '_USING_V110_SDK71_': False,
    '_WIN32_WINNT': False,
    '_WINDOWS': False,
    '__STD_C': False,
}

# -

print('\nRun actions')
required_files: Set[str] = set()

run_checked('ninja', '-C', str(OUT_DIR), ':commit_id')
required_files |= set(descs['//:commit_id']['outputs'])

# -

# Export our targets
print('\nExport targets')

# Clear our dest directories
targets_dir = pathlib.Path(GECKO_ANGLE_DIR, 'targets')
checkout_dir = pathlib.Path(GECKO_ANGLE_DIR, 'checkout')

shutil.rmtree(targets_dir, True)
shutil.rmtree(checkout_dir, True)
targets_dir.mkdir(exist_ok=True)
checkout_dir.mkdir(exist_ok=True)

# -

def export_target(target_name) -> Set[str]:
    #print(' ', target_name)
    desc = descs[target_name]
    flat = flattened_target(target_name, descs)
    assert target_name.startswith('//:'), target_name
    name = target_name[3:]

    required_files: Set[str] = set(flat['sources'])

    # Create our manifest lines
    target_dir = targets_dir / name
    target_dir.mkdir(exist_ok=True)

    lines = list(COMMON_HEADER)
    lines.append('')

    for x in sorted(set(desc['defines'])):
        try:
            (k, v) = x.split('=', 1)
            v = f"'{v}'"
        except ValueError:
            (k, v) = (x, 'True')
        try:
            line = f"DEFINES['{k}'] = {v}"
            if REGISTERED_DEFINES[k] == False:
                line = '#' + line
            lines.append(line)
        except KeyError:
            print(f'[{name}] Unrecognized define: {k}')
    lines.append('')

    cxxflags = set(desc['cflags'] + desc['cflags_cc'])

    def fixup_paths(listt):
        for x in set(listt):
            assert x.startswith('//'), x
            yield '../../checkout/' + x[2:]

    sources_by_config: Dict[str,List[str]] = {}
    extras: Dict[str,str] = dict()
    for x in fixup_paths(flat['sources']):
        #print(' '*5, x)
        (b, e) = x.rsplit('.', 1)
        if e in ['h', 'y', 'l', 'inc', 'inl']:
            continue
        elif e in ['cpp', 'cc', 'c']:
            if b.endswith('_win'):
                config = "CONFIG['OS_ARCH'] == 'WINNT'"
            elif b.endswith('_linux'):
                # Include these on BSDs too.
                config = "CONFIG['OS_ARCH'] not in ('Darwin', 'WINNT')"
            elif b.endswith('_mac'):
                config = "CONFIG['OS_ARCH'] == 'Darwin'"
            elif b.endswith('_posix'):
                config = "CONFIG['OS_ARCH'] != 'WINNT'"
            else:
                config = '' # None can't compare against str.

            sources_by_config.setdefault(config, []).append(x)
            continue
        elif e == 'rc':
            assert 'RCFILE' not in extras, (target_name, extras['RCFILE'], x)
            extras['RCFILE'] = f"'{x}'"
            continue
        elif e == 'def':
            assert 'DEFFILE' not in extras, (target_name, extras['DEFFILE'], x)
            extras['DEFFILE'] = f"'{x}'"
            continue
        else:
            assert False, ("Unhandled ext:", x)

    ldflags = set(desc['ldflags'])
    DEF_PREFIX = '/DEF:'
    for x in set(ldflags):
        if x.startswith(DEF_PREFIX):
            def_path = x[len(DEF_PREFIX):]
            required_files.add(def_path)
            assert 'DEFFILE' not in extras
            ldflags.remove(x)

            def_path = str(OUT_DIR) + '/' + def_path
            def_path = '//' + collapse_dotdots(def_path)

            def_rel_path = list(fixup_paths([def_path]))[0]
            extras['DEFFILE'] = "'{}'".format(def_rel_path)

    os_libs = list(map( lambda x: x[:-len('.lib')], set(desc.get('libs', [])) ))

    def append_arr_commented(dest, name, src):
        lines = []
        append_arr(lines, name, src)
        def comment(x):
            if x:
                x = '#' + x
            return x
        lines = map(comment, lines)
        dest += lines

    append_arr(lines, 'LOCAL_INCLUDES', fixup_paths(desc['include_dirs']))
    append_arr_commented(lines, 'CXXFLAGS', cxxflags)

    for (config,v) in sorted_items(sources_by_config):
        indent = 0
        if config:
            lines.append("if {}:".format(config))
            indent = 1
        append_arr(lines, 'SOURCES', v, indent=indent)

    dep_libs: Set[str] = set()
    for dep_name in set(flat['deps']):
        dep = descs[dep_name]
        if dep['type'] in LIBRARY_TYPES:
            assert dep_name.startswith('//:'), dep_name
            dep_libs.add(dep_name[3:])

    append_arr(lines, 'USE_LIBS', dep_libs)
    append_arr(lines, 'DIRS', ['../' + x for x in dep_libs])
    append_arr(lines, 'OS_LIBS', os_libs)
    append_arr_commented(lines, 'LDFLAGS', ldflags)

    for (k,v) in sorted(extras.items()):
        lines.append('{} = {}'.format(k, v))

    lib_type = desc['type']
    if lib_type == 'shared_library':
        lines.append(f"GeckoSharedLibrary('{name}', linkage=None)")
    elif lib_type == 'static_library':
        lines.append(f"Library('{name}')")
    else:
        assert False, lib_type

    # Write it out

    mozbuild = target_dir / 'moz.build'
    print(' ', ' ', f'Writing {mozbuild}')
    data = b'\n'.join((x.encode() for x in lines))
    mozbuild.write_bytes(data)

    return required_files

# -

for target_name in libraries:
    reqs = export_target(target_name)
    required_files |= reqs

# Copy all the files

print('\nMigrate required files')

i = 0
for x in required_files:
    i += 1
    sys.stdout.write(f'\r  Copying {i}/{len(required_files)}')
    sys.stdout.flush()
    assert x.startswith('//'), x
    x = x[2:]

    src = REPO_DIR / x
    dest = checkout_dir / x

    dest.parent.mkdir(parents=True, exist_ok=True)
    data = src.read_bytes()
    data = data.replace(b'\r\n', b'\n')
    dest.write_bytes(data)

print('\n\nDone')
