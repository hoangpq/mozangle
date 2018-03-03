// Generated from gfx/angle/**/moz.build by generate_build_data.py
// Do not edit directly. Instead, edit and run generate_build_data.py again.

pub struct Data {
     pub sources: &'static [&'static str],
     pub includes: &'static [&'static str],
     pub defines: &'static [(&'static str, Option<&'static str>)],
     pub os_libs: &'static [&'static str],
}

pub const TRANSLATOR: Data = Data {
    sources: &[
        "../../checkout/src/common/Float16ToFloat32.cpp",
        "../../checkout/src/common/MemoryBuffer.cpp",
        "../../checkout/src/common/aligned_memory.cpp",
        "../../checkout/src/common/angleutils.cpp",
        "../../checkout/src/common/debug.cpp",
        "../../checkout/src/common/mathutil.cpp",
        "../../checkout/src/common/matrix_utils.cpp",
        "../../checkout/src/common/string_utils.cpp",
        "../../checkout/src/common/system_utils.cpp",
        "../../checkout/src/common/third_party/base/anglebase/sha1.cc",
        "../../checkout/src/common/third_party/smhasher/src/PMurHash.cpp",
        "../../checkout/src/common/tls.cpp",
        "../../checkout/src/common/uniform_type_info_autogen.cpp",
        "../../checkout/src/common/utilities.cpp",
        "../../checkout/src/compiler/preprocessor/DiagnosticsBase.cpp",
        "../../checkout/src/compiler/preprocessor/DirectiveHandlerBase.cpp",
        "../../checkout/src/compiler/preprocessor/DirectiveParser.cpp",
        "../../checkout/src/compiler/preprocessor/ExpressionParser.cpp",
        "../../checkout/src/compiler/preprocessor/Input.cpp",
        "../../checkout/src/compiler/preprocessor/Lexer.cpp",
        "../../checkout/src/compiler/preprocessor/Macro.cpp",
        "../../checkout/src/compiler/preprocessor/MacroExpander.cpp",
        "../../checkout/src/compiler/preprocessor/Preprocessor.cpp",
        "../../checkout/src/compiler/preprocessor/Token.cpp",
        "../../checkout/src/compiler/preprocessor/Tokenizer.cpp",
        "../../checkout/src/compiler/translator/ASTMetadataHLSL.cpp",
        "../../checkout/src/compiler/translator/AddAndTrueToLoopCondition.cpp",
        "../../checkout/src/compiler/translator/AddDefaultReturnStatements.cpp",
        "../../checkout/src/compiler/translator/ArrayReturnValueToOutParameter.cpp",
        "../../checkout/src/compiler/translator/BreakVariableAliasingInInnerLoops.cpp",
        "../../checkout/src/compiler/translator/BuiltInFunctionEmulator.cpp",
        "../../checkout/src/compiler/translator/BuiltInFunctionEmulatorGLSL.cpp",
        "../../checkout/src/compiler/translator/BuiltInFunctionEmulatorHLSL.cpp",
        "../../checkout/src/compiler/translator/CallDAG.cpp",
        "../../checkout/src/compiler/translator/ClampFragDepth.cpp",
        "../../checkout/src/compiler/translator/ClampPointSize.cpp",
        "../../checkout/src/compiler/translator/CodeGen.cpp",
        "../../checkout/src/compiler/translator/CollectVariables.cpp",
        "../../checkout/src/compiler/translator/Compiler.cpp",
        "../../checkout/src/compiler/translator/ConstantUnion.cpp",
        "../../checkout/src/compiler/translator/Declarator.cpp",
        "../../checkout/src/compiler/translator/DeclareAndInitBuiltinsForInstancedMultiview.cpp",
        "../../checkout/src/compiler/translator/DeferGlobalInitializers.cpp",
        "../../checkout/src/compiler/translator/Diagnostics.cpp",
        "../../checkout/src/compiler/translator/DirectiveHandler.cpp",
        "../../checkout/src/compiler/translator/EmulateGLFragColorBroadcast.cpp",
        "../../checkout/src/compiler/translator/EmulatePrecision.cpp",
        "../../checkout/src/compiler/translator/ExpandIntegerPowExpressions.cpp",
        "../../checkout/src/compiler/translator/ExtensionBehavior.cpp",
        "../../checkout/src/compiler/translator/ExtensionGLSL.cpp",
        "../../checkout/src/compiler/translator/FindMain.cpp",
        "../../checkout/src/compiler/translator/FindSymbolNode.cpp",
        "../../checkout/src/compiler/translator/FlagStd140Structs.cpp",
        "../../checkout/src/compiler/translator/FoldExpressions.cpp",
        "../../checkout/src/compiler/translator/FunctionLookup.cpp",
        "../../checkout/src/compiler/translator/HashNames.cpp",
        "../../checkout/src/compiler/translator/ImageFunctionHLSL.cpp",
        "../../checkout/src/compiler/translator/ImmutableString.cpp",
        "../../checkout/src/compiler/translator/ImmutableStringBuilder.cpp",
        "../../checkout/src/compiler/translator/InfoSink.cpp",
        "../../checkout/src/compiler/translator/Initialize.cpp",
        "../../checkout/src/compiler/translator/InitializeDll.cpp",
        "../../checkout/src/compiler/translator/InitializeVariables.cpp",
        "../../checkout/src/compiler/translator/IntermNode.cpp",
        "../../checkout/src/compiler/translator/IntermNodePatternMatcher.cpp",
        "../../checkout/src/compiler/translator/IntermNode_util.cpp",
        "../../checkout/src/compiler/translator/IntermTraverse.cpp",
        "../../checkout/src/compiler/translator/IsASTDepthBelowLimit.cpp",
        "../../checkout/src/compiler/translator/Operator.cpp",
        "../../checkout/src/compiler/translator/OutputESSL.cpp",
        "../../checkout/src/compiler/translator/OutputGLSL.cpp",
        "../../checkout/src/compiler/translator/OutputGLSLBase.cpp",
        "../../checkout/src/compiler/translator/OutputHLSL.cpp",
        "../../checkout/src/compiler/translator/OutputTree.cpp",
        "../../checkout/src/compiler/translator/ParseContext.cpp",
        "../../checkout/src/compiler/translator/PoolAlloc.cpp",
        "../../checkout/src/compiler/translator/PruneNoOps.cpp",
        "../../checkout/src/compiler/translator/QualifierTypes.cpp",
        "../../checkout/src/compiler/translator/RecordConstantPrecision.cpp",
        "../../checkout/src/compiler/translator/RegenerateStructNames.cpp",
        "../../checkout/src/compiler/translator/RemoveArrayLengthMethod.cpp",
        "../../checkout/src/compiler/translator/RemoveDynamicIndexing.cpp",
        "../../checkout/src/compiler/translator/RemoveEmptySwitchStatements.cpp",
        "../../checkout/src/compiler/translator/RemoveInvariantDeclaration.cpp",
        "../../checkout/src/compiler/translator/RemoveNoOpCasesFromEndOfSwitchStatements.cpp",
        "../../checkout/src/compiler/translator/RemovePow.cpp",
        "../../checkout/src/compiler/translator/RemoveSwitchFallThrough.cpp",
        "../../checkout/src/compiler/translator/RemoveUnreferencedVariables.cpp",
        "../../checkout/src/compiler/translator/ReplaceVariable.cpp",
        "../../checkout/src/compiler/translator/RewriteDoWhile.cpp",
        "../../checkout/src/compiler/translator/RewriteElseBlocks.cpp",
        "../../checkout/src/compiler/translator/RewriteTexelFetchOffset.cpp",
        "../../checkout/src/compiler/translator/RewriteUnaryMinusOperatorFloat.cpp",
        "../../checkout/src/compiler/translator/RewriteUnaryMinusOperatorInt.cpp",
        "../../checkout/src/compiler/translator/RunAtTheEndOfShader.cpp",
        "../../checkout/src/compiler/translator/ScalarizeVecAndMatConstructorArgs.cpp",
        "../../checkout/src/compiler/translator/SeparateArrayConstructorStatements.cpp",
        "../../checkout/src/compiler/translator/SeparateArrayInitialization.cpp",
        "../../checkout/src/compiler/translator/SeparateDeclarations.cpp",
        "../../checkout/src/compiler/translator/SeparateExpressionsReturningArrays.cpp",
        "../../checkout/src/compiler/translator/ShaderLang.cpp",
        "../../checkout/src/compiler/translator/ShaderVars.cpp",
        "../../checkout/src/compiler/translator/SimplifyLoopConditions.cpp",
        "../../checkout/src/compiler/translator/SplitSequenceOperator.cpp",
        "../../checkout/src/compiler/translator/StaticType.cpp",
        "../../checkout/src/compiler/translator/StructureHLSL.cpp",
        "../../checkout/src/compiler/translator/Symbol.cpp",
        "../../checkout/src/compiler/translator/SymbolTable.cpp",
        "../../checkout/src/compiler/translator/SymbolUniqueId.cpp",
        "../../checkout/src/compiler/translator/TextureFunctionHLSL.cpp",
        "../../checkout/src/compiler/translator/TranslatorESSL.cpp",
        "../../checkout/src/compiler/translator/TranslatorGLSL.cpp",
        "../../checkout/src/compiler/translator/TranslatorHLSL.cpp",
        "../../checkout/src/compiler/translator/Types.cpp",
        "../../checkout/src/compiler/translator/UnfoldShortCircuitAST.cpp",
        "../../checkout/src/compiler/translator/UnfoldShortCircuitToIf.cpp",
        "../../checkout/src/compiler/translator/UniformHLSL.cpp",
        "../../checkout/src/compiler/translator/UseInterfaceBlockFields.cpp",
        "../../checkout/src/compiler/translator/UtilsHLSL.cpp",
        "../../checkout/src/compiler/translator/ValidateGlobalInitializer.cpp",
        "../../checkout/src/compiler/translator/ValidateLimitations.cpp",
        "../../checkout/src/compiler/translator/ValidateMaxParameters.cpp",
        "../../checkout/src/compiler/translator/ValidateOutputs.cpp",
        "../../checkout/src/compiler/translator/ValidateSwitch.cpp",
        "../../checkout/src/compiler/translator/ValidateVaryingLocations.cpp",
        "../../checkout/src/compiler/translator/VariablePacker.cpp",
        "../../checkout/src/compiler/translator/VectorizeVectorScalarArithmetic.cpp",
        "../../checkout/src/compiler/translator/VersionGLSL.cpp",
        "../../checkout/src/compiler/translator/WrapSwitchStatementsInBlocks.cpp",
        "../../checkout/src/compiler/translator/blocklayout.cpp",
        "../../checkout/src/compiler/translator/blocklayoutHLSL.cpp",
        "../../checkout/src/compiler/translator/emulated_builtin_functions_hlsl_autogen.cpp",
        "../../checkout/src/compiler/translator/glslang_lex.cpp",
        "../../checkout/src/compiler/translator/glslang_tab.cpp",
        "../../checkout/src/compiler/translator/util.cpp",
        "../../checkout/src/third_party/compiler/ArrayBoundsClamper.cpp",
    ],
    includes: &[
        "../../checkout/",
        "../../checkout/include/",
        "../../checkout/out/gen/",
        "../../checkout/out/gen/angle/",
        "../../checkout/src/",
        "../../checkout/src/common/third_party/base/",
    ],
    defines: &[
        ("ANGLE_ENABLE_DEBUG_ANNOTATIONS", None),
        ("ANGLE_ENABLE_ESSL", None),
        ("ANGLE_ENABLE_GLSL", None),
        ("ANGLE_ENABLE_HLSL", None),
        ("ANGLE_SKIP_DXGI_1_2_CHECK", None),
        ("DYNAMIC_ANNOTATIONS_ENABLED", Some("1")),
        ("EGL_EGLEXT_PROTOTYPES", None),
        ("GL_GLEXT_PROTOTYPES", None),
        ("NOMINMAX", None),
        ("NTDDI_VERSION", Some("0x0A000000")),
        ("UNICODE", None),
        ("_ATL_NO_OPENGL", None),
        ("_CRT_RAND_S", None),
        ("_CRT_SECURE_NO_DEPRECATE", None),
        ("_HAS_EXCEPTIONS", Some("0")),
        ("_SCL_SECURE_NO_DEPRECATE", None),
        ("_SECURE_ATL", None),
        ("_UNICODE", None),
        ("__NDK_FPABI__", Some("")),
        ("constexpr14", Some("")),
    ],
    os_libs: &[
    ],
};
pub const EGL: Data = Data {
    sources: &[
        "../../checkout/src/common/Float16ToFloat32.cpp",
        "../../checkout/src/common/MemoryBuffer.cpp",
        "../../checkout/src/common/aligned_memory.cpp",
        "../../checkout/src/common/angleutils.cpp",
        "../../checkout/src/common/debug.cpp",
        "../../checkout/src/common/event_tracer.cpp",
        "../../checkout/src/common/mathutil.cpp",
        "../../checkout/src/common/matrix_utils.cpp",
        "../../checkout/src/common/string_utils.cpp",
        "../../checkout/src/common/system_utils.cpp",
        "../../checkout/src/common/third_party/base/anglebase/sha1.cc",
        "../../checkout/src/common/third_party/smhasher/src/PMurHash.cpp",
        "../../checkout/src/common/tls.cpp",
        "../../checkout/src/common/uniform_type_info_autogen.cpp",
        "../../checkout/src/common/utilities.cpp",
        "../../checkout/src/compiler/preprocessor/DiagnosticsBase.cpp",
        "../../checkout/src/compiler/preprocessor/DirectiveHandlerBase.cpp",
        "../../checkout/src/compiler/preprocessor/DirectiveParser.cpp",
        "../../checkout/src/compiler/preprocessor/ExpressionParser.cpp",
        "../../checkout/src/compiler/preprocessor/Input.cpp",
        "../../checkout/src/compiler/preprocessor/Lexer.cpp",
        "../../checkout/src/compiler/preprocessor/Macro.cpp",
        "../../checkout/src/compiler/preprocessor/MacroExpander.cpp",
        "../../checkout/src/compiler/preprocessor/Preprocessor.cpp",
        "../../checkout/src/compiler/preprocessor/Token.cpp",
        "../../checkout/src/compiler/preprocessor/Tokenizer.cpp",
        "../../checkout/src/compiler/translator/ASTMetadataHLSL.cpp",
        "../../checkout/src/compiler/translator/AddAndTrueToLoopCondition.cpp",
        "../../checkout/src/compiler/translator/AddDefaultReturnStatements.cpp",
        "../../checkout/src/compiler/translator/ArrayReturnValueToOutParameter.cpp",
        "../../checkout/src/compiler/translator/BreakVariableAliasingInInnerLoops.cpp",
        "../../checkout/src/compiler/translator/BuiltInFunctionEmulator.cpp",
        "../../checkout/src/compiler/translator/BuiltInFunctionEmulatorGLSL.cpp",
        "../../checkout/src/compiler/translator/BuiltInFunctionEmulatorHLSL.cpp",
        "../../checkout/src/compiler/translator/CallDAG.cpp",
        "../../checkout/src/compiler/translator/ClampFragDepth.cpp",
        "../../checkout/src/compiler/translator/ClampPointSize.cpp",
        "../../checkout/src/compiler/translator/CodeGen.cpp",
        "../../checkout/src/compiler/translator/CollectVariables.cpp",
        "../../checkout/src/compiler/translator/Compiler.cpp",
        "../../checkout/src/compiler/translator/ConstantUnion.cpp",
        "../../checkout/src/compiler/translator/Declarator.cpp",
        "../../checkout/src/compiler/translator/DeclareAndInitBuiltinsForInstancedMultiview.cpp",
        "../../checkout/src/compiler/translator/DeferGlobalInitializers.cpp",
        "../../checkout/src/compiler/translator/Diagnostics.cpp",
        "../../checkout/src/compiler/translator/DirectiveHandler.cpp",
        "../../checkout/src/compiler/translator/EmulateGLFragColorBroadcast.cpp",
        "../../checkout/src/compiler/translator/EmulatePrecision.cpp",
        "../../checkout/src/compiler/translator/ExpandIntegerPowExpressions.cpp",
        "../../checkout/src/compiler/translator/ExtensionBehavior.cpp",
        "../../checkout/src/compiler/translator/ExtensionGLSL.cpp",
        "../../checkout/src/compiler/translator/FindMain.cpp",
        "../../checkout/src/compiler/translator/FindSymbolNode.cpp",
        "../../checkout/src/compiler/translator/FlagStd140Structs.cpp",
        "../../checkout/src/compiler/translator/FoldExpressions.cpp",
        "../../checkout/src/compiler/translator/FunctionLookup.cpp",
        "../../checkout/src/compiler/translator/HashNames.cpp",
        "../../checkout/src/compiler/translator/ImageFunctionHLSL.cpp",
        "../../checkout/src/compiler/translator/ImmutableString.cpp",
        "../../checkout/src/compiler/translator/ImmutableStringBuilder.cpp",
        "../../checkout/src/compiler/translator/InfoSink.cpp",
        "../../checkout/src/compiler/translator/Initialize.cpp",
        "../../checkout/src/compiler/translator/InitializeDll.cpp",
        "../../checkout/src/compiler/translator/InitializeVariables.cpp",
        "../../checkout/src/compiler/translator/IntermNode.cpp",
        "../../checkout/src/compiler/translator/IntermNodePatternMatcher.cpp",
        "../../checkout/src/compiler/translator/IntermNode_util.cpp",
        "../../checkout/src/compiler/translator/IntermTraverse.cpp",
        "../../checkout/src/compiler/translator/IsASTDepthBelowLimit.cpp",
        "../../checkout/src/compiler/translator/Operator.cpp",
        "../../checkout/src/compiler/translator/OutputESSL.cpp",
        "../../checkout/src/compiler/translator/OutputGLSL.cpp",
        "../../checkout/src/compiler/translator/OutputGLSLBase.cpp",
        "../../checkout/src/compiler/translator/OutputHLSL.cpp",
        "../../checkout/src/compiler/translator/OutputTree.cpp",
        "../../checkout/src/compiler/translator/ParseContext.cpp",
        "../../checkout/src/compiler/translator/PoolAlloc.cpp",
        "../../checkout/src/compiler/translator/PruneNoOps.cpp",
        "../../checkout/src/compiler/translator/QualifierTypes.cpp",
        "../../checkout/src/compiler/translator/RecordConstantPrecision.cpp",
        "../../checkout/src/compiler/translator/RegenerateStructNames.cpp",
        "../../checkout/src/compiler/translator/RemoveArrayLengthMethod.cpp",
        "../../checkout/src/compiler/translator/RemoveDynamicIndexing.cpp",
        "../../checkout/src/compiler/translator/RemoveEmptySwitchStatements.cpp",
        "../../checkout/src/compiler/translator/RemoveInvariantDeclaration.cpp",
        "../../checkout/src/compiler/translator/RemoveNoOpCasesFromEndOfSwitchStatements.cpp",
        "../../checkout/src/compiler/translator/RemovePow.cpp",
        "../../checkout/src/compiler/translator/RemoveSwitchFallThrough.cpp",
        "../../checkout/src/compiler/translator/RemoveUnreferencedVariables.cpp",
        "../../checkout/src/compiler/translator/ReplaceVariable.cpp",
        "../../checkout/src/compiler/translator/RewriteDoWhile.cpp",
        "../../checkout/src/compiler/translator/RewriteElseBlocks.cpp",
        "../../checkout/src/compiler/translator/RewriteTexelFetchOffset.cpp",
        "../../checkout/src/compiler/translator/RewriteUnaryMinusOperatorFloat.cpp",
        "../../checkout/src/compiler/translator/RewriteUnaryMinusOperatorInt.cpp",
        "../../checkout/src/compiler/translator/RunAtTheEndOfShader.cpp",
        "../../checkout/src/compiler/translator/ScalarizeVecAndMatConstructorArgs.cpp",
        "../../checkout/src/compiler/translator/SeparateArrayConstructorStatements.cpp",
        "../../checkout/src/compiler/translator/SeparateArrayInitialization.cpp",
        "../../checkout/src/compiler/translator/SeparateDeclarations.cpp",
        "../../checkout/src/compiler/translator/SeparateExpressionsReturningArrays.cpp",
        "../../checkout/src/compiler/translator/ShaderLang.cpp",
        "../../checkout/src/compiler/translator/ShaderVars.cpp",
        "../../checkout/src/compiler/translator/SimplifyLoopConditions.cpp",
        "../../checkout/src/compiler/translator/SplitSequenceOperator.cpp",
        "../../checkout/src/compiler/translator/StaticType.cpp",
        "../../checkout/src/compiler/translator/StructureHLSL.cpp",
        "../../checkout/src/compiler/translator/Symbol.cpp",
        "../../checkout/src/compiler/translator/SymbolTable.cpp",
        "../../checkout/src/compiler/translator/SymbolUniqueId.cpp",
        "../../checkout/src/compiler/translator/TextureFunctionHLSL.cpp",
        "../../checkout/src/compiler/translator/TranslatorESSL.cpp",
        "../../checkout/src/compiler/translator/TranslatorGLSL.cpp",
        "../../checkout/src/compiler/translator/TranslatorHLSL.cpp",
        "../../checkout/src/compiler/translator/Types.cpp",
        "../../checkout/src/compiler/translator/UnfoldShortCircuitAST.cpp",
        "../../checkout/src/compiler/translator/UnfoldShortCircuitToIf.cpp",
        "../../checkout/src/compiler/translator/UniformHLSL.cpp",
        "../../checkout/src/compiler/translator/UseInterfaceBlockFields.cpp",
        "../../checkout/src/compiler/translator/UtilsHLSL.cpp",
        "../../checkout/src/compiler/translator/ValidateGlobalInitializer.cpp",
        "../../checkout/src/compiler/translator/ValidateLimitations.cpp",
        "../../checkout/src/compiler/translator/ValidateMaxParameters.cpp",
        "../../checkout/src/compiler/translator/ValidateOutputs.cpp",
        "../../checkout/src/compiler/translator/ValidateSwitch.cpp",
        "../../checkout/src/compiler/translator/ValidateVaryingLocations.cpp",
        "../../checkout/src/compiler/translator/VariablePacker.cpp",
        "../../checkout/src/compiler/translator/VectorizeVectorScalarArithmetic.cpp",
        "../../checkout/src/compiler/translator/VersionGLSL.cpp",
        "../../checkout/src/compiler/translator/WrapSwitchStatementsInBlocks.cpp",
        "../../checkout/src/compiler/translator/blocklayout.cpp",
        "../../checkout/src/compiler/translator/blocklayoutHLSL.cpp",
        "../../checkout/src/compiler/translator/emulated_builtin_functions_hlsl_autogen.cpp",
        "../../checkout/src/compiler/translator/glslang_lex.cpp",
        "../../checkout/src/compiler/translator/glslang_tab.cpp",
        "../../checkout/src/compiler/translator/util.cpp",
        "../../checkout/src/gpu_info_util/SystemInfo.cpp",
        "../../checkout/src/image_util/copyimage.cpp",
        "../../checkout/src/image_util/imageformats.cpp",
        "../../checkout/src/image_util/loadimage.cpp",
        "../../checkout/src/image_util/loadimage_etc.cpp",
        "../../checkout/src/libANGLE/AttributeMap.cpp",
        "../../checkout/src/libANGLE/Buffer.cpp",
        "../../checkout/src/libANGLE/Caps.cpp",
        "../../checkout/src/libANGLE/Compiler.cpp",
        "../../checkout/src/libANGLE/Config.cpp",
        "../../checkout/src/libANGLE/Context.cpp",
        "../../checkout/src/libANGLE/ContextState.cpp",
        "../../checkout/src/libANGLE/Debug.cpp",
        "../../checkout/src/libANGLE/Device.cpp",
        "../../checkout/src/libANGLE/Display.cpp",
        "../../checkout/src/libANGLE/Error.cpp",
        "../../checkout/src/libANGLE/Fence.cpp",
        "../../checkout/src/libANGLE/Framebuffer.cpp",
        "../../checkout/src/libANGLE/FramebufferAttachment.cpp",
        "../../checkout/src/libANGLE/HandleAllocator.cpp",
        "../../checkout/src/libANGLE/HandleRangeAllocator.cpp",
        "../../checkout/src/libANGLE/Image.cpp",
        "../../checkout/src/libANGLE/ImageIndex.cpp",
        "../../checkout/src/libANGLE/IndexRangeCache.cpp",
        "../../checkout/src/libANGLE/LoggingAnnotator.cpp",
        "../../checkout/src/libANGLE/MemoryProgramCache.cpp",
        "../../checkout/src/libANGLE/PackedGLEnums_autogen.cpp",
        "../../checkout/src/libANGLE/Path.cpp",
        "../../checkout/src/libANGLE/Platform.cpp",
        "../../checkout/src/libANGLE/Program.cpp",
        "../../checkout/src/libANGLE/ProgramLinkedResources.cpp",
        "../../checkout/src/libANGLE/ProgramPipeline.cpp",
        "../../checkout/src/libANGLE/Query.cpp",
        "../../checkout/src/libANGLE/Renderbuffer.cpp",
        "../../checkout/src/libANGLE/ResourceManager.cpp",
        "../../checkout/src/libANGLE/Sampler.cpp",
        "../../checkout/src/libANGLE/Shader.cpp",
        "../../checkout/src/libANGLE/State.cpp",
        "../../checkout/src/libANGLE/Stream.cpp",
        "../../checkout/src/libANGLE/Surface.cpp",
        "../../checkout/src/libANGLE/Texture.cpp",
        "../../checkout/src/libANGLE/Thread.cpp",
        "../../checkout/src/libANGLE/TransformFeedback.cpp",
        "../../checkout/src/libANGLE/Uniform.cpp",
        "../../checkout/src/libANGLE/VaryingPacking.cpp",
        "../../checkout/src/libANGLE/VertexArray.cpp",
        "../../checkout/src/libANGLE/VertexAttribute.cpp",
        "../../checkout/src/libANGLE/WorkerThread.cpp",
        "../../checkout/src/libANGLE/angletypes.cpp",
        "../../checkout/src/libANGLE/es3_copy_conversion_table_autogen.cpp",
        "../../checkout/src/libANGLE/format_map_autogen.cpp",
        "../../checkout/src/libANGLE/formatutils.cpp",
        "../../checkout/src/libANGLE/params.cpp",
        "../../checkout/src/libANGLE/queryconversions.cpp",
        "../../checkout/src/libANGLE/queryutils.cpp",
        "../../checkout/src/libANGLE/renderer/ContextImpl.cpp",
        "../../checkout/src/libANGLE/renderer/DeviceImpl.cpp",
        "../../checkout/src/libANGLE/renderer/DisplayImpl.cpp",
        "../../checkout/src/libANGLE/renderer/Format_table_autogen.cpp",
        "../../checkout/src/libANGLE/renderer/SurfaceImpl.cpp",
        "../../checkout/src/libANGLE/renderer/TextureImpl.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/BufferD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/CompilerD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/DeviceD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/DisplayD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/DynamicHLSL.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/EGLImageD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/FramebufferD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/HLSLCompiler.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/ImageD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/IndexBuffer.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/IndexDataManager.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/NativeWindowD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/ProgramD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/RenderTargetD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/RenderbufferD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/RendererD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/ShaderD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/ShaderExecutableD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/SurfaceD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/SwapChainD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/TextureD3D.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/VertexBuffer.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/VertexDataManager.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/Blit11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/Buffer11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/Clear11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/Context11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/DebugAnnotator11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/Fence11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/Framebuffer11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/Image11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/IndexBuffer11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/InputLayoutCache.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/PixelTransfer11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/ProgramPipeline11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/Query11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/RenderStateCache.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/RenderTarget11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/Renderer11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/ResourceManager11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/ShaderExecutable11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/StateManager11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/StreamProducerD3DTexture.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/SwapChain11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/TextureStorage11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/TransformFeedback11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/Trim11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/VertexArray11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/VertexBuffer11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/dxgi_format_map_autogen.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/dxgi_support_table.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/formatutils11.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/renderer11_utils.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/texture_format_table.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/texture_format_table_autogen.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d11/win32/NativeWindow11Win32.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/Blit9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/Buffer9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/Context9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/DebugAnnotator9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/Fence9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/Framebuffer9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/Image9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/IndexBuffer9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/NativeWindow9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/Query9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/RenderTarget9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/Renderer9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/ShaderExecutable9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/StateManager9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/SwapChain9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/TextureStorage9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/VertexBuffer9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/VertexDeclarationCache.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/formatutils9.cpp",
        "../../checkout/src/libANGLE/renderer/d3d/d3d9/renderer9_utils.cpp",
        "../../checkout/src/libANGLE/renderer/driver_utils.cpp",
        "../../checkout/src/libANGLE/renderer/load_functions_table_autogen.cpp",
        "../../checkout/src/libANGLE/renderer/renderer_utils.cpp",
        "../../checkout/src/libANGLE/signal_utils.cpp",
        "../../checkout/src/libANGLE/validationEGL.cpp",
        "../../checkout/src/libANGLE/validationES.cpp",
        "../../checkout/src/libANGLE/validationES1.cpp",
        "../../checkout/src/libANGLE/validationES2.cpp",
        "../../checkout/src/libANGLE/validationES3.cpp",
        "../../checkout/src/libANGLE/validationES31.cpp",
        "../../checkout/src/libEGL/libEGL.cpp",
        "../../checkout/src/libGLESv2/entry_points_egl.cpp",
        "../../checkout/src/libGLESv2/entry_points_egl_ext.cpp",
        "../../checkout/src/libGLESv2/entry_points_gles_1_0_autogen.cpp",
        "../../checkout/src/libGLESv2/entry_points_gles_2_0_autogen.cpp",
        "../../checkout/src/libGLESv2/entry_points_gles_2_0_ext.cpp",
        "../../checkout/src/libGLESv2/entry_points_gles_3_0_autogen.cpp",
        "../../checkout/src/libGLESv2/entry_points_gles_3_1_autogen.cpp",
        "../../checkout/src/libGLESv2/entry_points_gles_ext_autogen.cpp",
        "../../checkout/src/libGLESv2/global_state.cpp",
        "../../checkout/src/libGLESv2/libGLESv2.cpp",
        "../../checkout/src/libGLESv2/proc_table_autogen.cpp",
        "../../checkout/src/third_party/compiler/ArrayBoundsClamper.cpp",
        "../../checkout/src/third_party/systeminfo/SystemInfo.cpp",
    ],
    includes: &[
        "../../checkout/",
        "../../checkout/include/",
        "../../checkout/out/gen/",
        "../../checkout/out/gen/angle/",
        "../../checkout/src/",
        "../../checkout/src/common/third_party/base/",
    ],
    defines: &[
        ("ANGLE_ENABLE_D3D11", None),
        ("ANGLE_ENABLE_D3D9", None),
        ("ANGLE_ENABLE_DEBUG_ANNOTATIONS", None),
        ("ANGLE_ENABLE_ESSL", None),
        ("ANGLE_ENABLE_GLSL", None),
        ("ANGLE_ENABLE_HLSL", None),
        ("ANGLE_GENERATE_SHADER_DEBUG_INFO", None),
        ("ANGLE_SKIP_DXGI_1_2_CHECK", None),
        ("DYNAMIC_ANNOTATIONS_ENABLED", Some("1")),
        ("EGLAPI", Some("")),
        ("EGL_EGLEXT_PROTOTYPES", None),
        ("GL_API", Some("")),
        ("GL_APICALL", Some("")),
        ("GL_GLEXT_PROTOTYPES", None),
        ("GPU_INFO_USE_SETUPAPI", None),
        ("LIBANGLE_IMPLEMENTATION", None),
        ("LIBEGL_IMPLEMENTATION", None),
        ("LIBGLESV2_IMPLEMENTATION", None),
        ("NOMINMAX", None),
        ("NTDDI_VERSION", Some("0x0A000000")),
        ("UNICODE", None),
        ("_ATL_NO_OPENGL", None),
        ("_CRT_RAND_S", None),
        ("_CRT_SECURE_NO_DEPRECATE", None),
        ("_HAS_EXCEPTIONS", Some("0")),
        ("_SCL_SECURE_NO_DEPRECATE", None),
        ("_SECURE_ATL", None),
        ("_UNICODE", None),
        ("__NDK_FPABI__", Some("")),
        ("constexpr14", Some("")),
    ],
    os_libs: &[
        "advapi32",
        "comdlg32",
        "d3d9",
        "dbghelp",
        "delayimp",
        "dnsapi",
        "dxguid",
        "gdi32",
        "kernel32",
        "msimg32",
        "odbc32",
        "odbccp32",
        "ole32",
        "oleaut32",
        "psapi",
        "setupapi",
        "shell32",
        "shlwapi",
        "user32",
        "usp10",
        "uuid",
        "version",
        "wininet",
        "winmm",
        "winspool",
        "ws2_32",
    ],
};
