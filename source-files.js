var sourcesIndex = JSON.parse('{\
"adler":["",[],["algo.rs","lib.rs"]],\
"aho_corasick":["",[["packed",[["teddy",[],["compile.rs","mod.rs","runtime.rs"]]],["api.rs","mod.rs","pattern.rs","rabinkarp.rs","vector.rs"]]],["ahocorasick.rs","automaton.rs","buffer.rs","byte_frequencies.rs","classes.rs","dfa.rs","error.rs","lib.rs","nfa.rs","prefilter.rs","state_id.rs"]],\
"ansi_term":["",[],["ansi.rs","debug.rs","difference.rs","display.rs","lib.rs","style.rs","util.rs","windows.rs","write.rs"]],\
"anyhow":["",[],["backtrace.rs","chain.rs","context.rs","ensure.rs","error.rs","fmt.rs","kind.rs","lib.rs","macros.rs","ptr.rs","wrapper.rs"]],\
"approx":["",[],["abs_diff_eq.rs","lib.rs","macros.rs","relative_eq.rs","ulps_eq.rs"]],\
"aquamarine":["",[],["attrs.rs","lib.rs","parse.rs"]],\
"ash":["",[["extensions",[["experimental",[],["amd.rs","mod.rs"]],["ext",[],["buffer_device_address.rs","calibrated_timestamps.rs","debug_marker.rs","debug_report.rs","debug_utils.rs","extended_dynamic_state.rs","extended_dynamic_state2.rs","full_screen_exclusive.rs","headless_surface.rs","metal_surface.rs","mod.rs","physical_device_drm.rs","private_data.rs","tooling_info.rs"]],["khr",[],["acceleration_structure.rs","android_surface.rs","buffer_device_address.rs","copy_commands2.rs","create_render_pass2.rs","deferred_host_operations.rs","display.rs","display_swapchain.rs","draw_indirect_count.rs","dynamic_rendering.rs","external_fence_fd.rs","external_fence_win32.rs","external_memory_fd.rs","external_memory_win32.rs","external_semaphore_fd.rs","external_semaphore_win32.rs","get_memory_requirements2.rs","get_physical_device_properties2.rs","get_surface_capabilities2.rs","maintenance1.rs","maintenance3.rs","maintenance4.rs","mod.rs","pipeline_executable_properties.rs","present_wait.rs","push_descriptor.rs","ray_tracing_pipeline.rs","surface.rs","swapchain.rs","synchronization2.rs","timeline_semaphore.rs","wayland_surface.rs","win32_surface.rs","xcb_surface.rs","xlib_surface.rs"]],["mvk",[],["ios_surface.rs","macos_surface.rs","mod.rs"]],["nn",[],["mod.rs","vi_surface.rs"]],["nv",[],["device_diagnostic_checkpoints.rs","mesh_shader.rs","mod.rs","ray_tracing.rs"]]],["mod.rs"]],["vk",[],["aliases.rs","bitflags.rs","const_debugs.rs","constants.rs","definitions.rs","enums.rs","extensions.rs","feature_extensions.rs","features.rs","macros.rs","native.rs","platform_types.rs","prelude.rs"]]],["device.rs","entry.rs","instance.rs","lib.rs","prelude.rs","util.rs","vk.rs"]],\
"atty":["",[],["lib.rs"]],\
"bit_field":["",[],["lib.rs"]],\
"bitflags":["",[],["lib.rs"]],\
"bytemuck":["",[],["allocation.rs","anybitpattern.rs","checked.rs","contiguous.rs","internal.rs","lib.rs","no_uninit.rs","offset_of.rs","pod.rs","pod_in_option.rs","transparent.rs","zeroable.rs","zeroable_in_option.rs"]],\
"byteorder":["",[],["io.rs","lib.rs"]],\
"ccthw_ash_allocator":["",[["allocation_requirements",[],["dedicated_resource_handle.rs","mod.rs"]],["memory_allocator",[["page_suballocator",[],["mod.rs","page_arena.rs"]]],["composable_allocator.rs","dedicated_allocator.rs","device_allocator.rs","fake_allocator.rs","memory_type_pool_allocator.rs","mod.rs","pool_allocator.rs","sized_allocator.rs","trace_allocator.rs"]]],["allocation.rs","device_memory.rs","error.rs","lib.rs","memory_properties.rs","pretty_wrappers.rs"]],\
"ccthw_ash_instance":["",[["logical_device",[],["mod.rs","queue_family_info.rs"]],["physical_device",[["physical_device_features",[],["is_supported_by.rs","mod.rs"]]],["mod.rs","physical_device_properties.rs"]],["vulkan_instance",[],["create_instance.rs","debug_callback.rs","mod.rs"]]],["error.rs","ffi.rs","lib.rs"]],\
"cfg_if":["",[],["lib.rs"]],\
"chrono":["",[["datetime",[],["mod.rs"]],["format",[],["mod.rs","parse.rs","parsed.rs","scan.rs","strftime.rs"]],["naive",[["datetime",[],["mod.rs"]],["time",[],["mod.rs"]]],["date.rs","internals.rs","isoweek.rs","mod.rs"]],["offset",[["local",[["tz_info",[],["mod.rs","parser.rs","rule.rs","timezone.rs"]]],["mod.rs","unix.rs"]]],["fixed.rs","mod.rs","utc.rs"]]],["date.rs","lib.rs","month.rs","oldtime.rs","round.rs","traits.rs","weekday.rs"]],\
"color_quant":["",[],["lib.rs","math.rs"]],\
"crc32fast":["",[["specialized",[],["mod.rs","pclmulqdq.rs"]]],["baseline.rs","combine.rs","lib.rs","table.rs"]],\
"crossbeam_channel":["",[["flavors",[],["array.rs","at.rs","list.rs","mod.rs","never.rs","tick.rs","zero.rs"]]],["channel.rs","context.rs","counter.rs","err.rs","lib.rs","select.rs","select_macro.rs","utils.rs","waker.rs"]],\
"crossbeam_deque":["",[],["deque.rs","lib.rs"]],\
"crossbeam_epoch":["",[["sync",[],["list.rs","mod.rs","once_lock.rs","queue.rs"]]],["atomic.rs","collector.rs","default.rs","deferred.rs","epoch.rs","guard.rs","internal.rs","lib.rs"]],\
"crossbeam_queue":["",[],["array_queue.rs","lib.rs","seg_queue.rs"]],\
"crossbeam_utils":["",[["atomic",[],["atomic_cell.rs","consume.rs","mod.rs","seq_lock.rs"]],["sync",[],["mod.rs","once_lock.rs","parker.rs","sharded_lock.rs","wait_group.rs"]]],["backoff.rs","cache_padded.rs","lib.rs","thread.rs"]],\
"either":["",[],["lib.rs"]],\
"exr":["",[["block",[],["chunk.rs","lines.rs","mod.rs","reader.rs","samples.rs","writer.rs"]],["compression",[["b44",[],["mod.rs","table.rs"]],["piz",[],["huffman.rs","mod.rs","wavelet.rs"]]],["mod.rs","pxr24.rs","rle.rs","zip.rs"]],["image",[["read",[],["any_channels.rs","image.rs","layers.rs","levels.rs","mod.rs","samples.rs","specific_channels.rs"]],["write",[],["channels.rs","layers.rs","mod.rs","samples.rs"]]],["crop.rs","mod.rs","pixel_vec.rs","recursive.rs"]],["meta",[],["attribute.rs","header.rs","mod.rs"]]],["error.rs","io.rs","lib.rs","math.rs"]],\
"flate2":["",[["deflate",[],["bufread.rs","mod.rs","read.rs","write.rs"]],["ffi",[],["mod.rs","rust.rs"]],["gz",[],["bufread.rs","mod.rs","read.rs","write.rs"]],["zlib",[],["bufread.rs","mod.rs","read.rs","write.rs"]]],["bufreader.rs","crc.rs","lib.rs","mem.rs","zio.rs"]],\
"flexi_logger":["",[["primary_writer",[],["multi_writer.rs","std_stream.rs","std_writer.rs"]],["writers",[["file_log_writer",[],["builder.rs","config.rs","state.rs","state_handle.rs","threads.rs"]]],["file_log_writer.rs","log_writer.rs"]]],["code_examples.rs","deferred_now.rs","error_info.rs","file_spec.rs","filter.rs","flexi_error.rs","flexi_logger.rs","formats.rs","lib.rs","log_specification.rs","logger.rs","logger_handle.rs","parameters.rs","primary_writer.rs","threads.rs","util.rs","write_mode.rs","writers.rs"]],\
"flume":["",[],["async.rs","lib.rs","select.rs","signal.rs"]],\
"futures_core":["",[["task",[["__internal",[],["atomic_waker.rs","mod.rs"]]],["mod.rs","poll.rs"]]],["future.rs","lib.rs","stream.rs"]],\
"futures_sink":["",[],["lib.rs"]],\
"getrandom":["",[],["error.rs","error_impls.rs","lib.rs","linux_android.rs","use_file.rs","util.rs","util_libc.rs"]],\
"gif":["",[["reader",[],["decoder.rs","mod.rs"]]],["common.rs","encoder.rs","lib.rs","traits.rs"]],\
"glfw":["",[["ffi",[],["link.rs","mod.rs"]]],["callbacks.rs","lib.rs"]],\
"glfw_sys":["",[],["lib.rs"]],\
"glob":["",[],["lib.rs"]],\
"half":["",[["bfloat",[],["convert.rs"]],["binary16",[],["convert.rs"]]],["bfloat.rs","binary16.rs","lib.rs","slice.rs"]],\
"iana_time_zone":["",[],["lib.rs","tz_linux.rs"]],\
"image":["",[["codecs",[["bmp",[],["decoder.rs","encoder.rs","mod.rs"]],["hdr",[],["decoder.rs","encoder.rs","mod.rs"]],["ico",[],["decoder.rs","encoder.rs","mod.rs"]],["jpeg",[],["decoder.rs","encoder.rs","entropy.rs","mod.rs","transform.rs"]],["pnm",[],["autobreak.rs","decoder.rs","encoder.rs","header.rs","mod.rs"]],["tga",[],["decoder.rs","encoder.rs","header.rs","mod.rs"]],["webp",[],["decoder.rs","extended.rs","huffman.rs","loop_filter.rs","lossless.rs","lossless_transform.rs","mod.rs","transform.rs","vp8.rs"]]],["dds.rs","dxt.rs","farbfeld.rs","gif.rs","openexr.rs","png.rs","tiff.rs"]],["imageops",[],["affine.rs","colorops.rs","mod.rs","sample.rs"]],["io",[],["free_functions.rs","mod.rs","reader.rs"]],["math",[],["mod.rs","rect.rs","utils.rs"]],["utils",[],["mod.rs"]]],["animation.rs","buffer.rs","color.rs","dynimage.rs","error.rs","flat.rs","image.rs","lib.rs","traits.rs"]],\
"indoc":["",[],["error.rs","expr.rs","lib.rs","unindent.rs"]],\
"itertools":["",[["adaptors",[],["mod.rs","multi_product.rs"]]],["combinations.rs","combinations_with_replacement.rs","concat_impl.rs","cons_tuples_impl.rs","diff.rs","either_or_both.rs","exactly_one_err.rs","format.rs","free.rs","group_map.rs","groupbylazy.rs","impl_macros.rs","intersperse.rs","kmerge_impl.rs","lazy_buffer.rs","lib.rs","merge_join.rs","minmax.rs","multipeek_impl.rs","pad_tail.rs","peeking_take_while.rs","permutations.rs","process_results_impl.rs","put_back_n_impl.rs","rciter_impl.rs","repeatn.rs","size_hint.rs","sources.rs","tee.rs","tuple_impl.rs","unique_impl.rs","with_position.rs","zip_eq_impl.rs","zip_longest.rs","ziptuple.rs"]],\
"jpeg_decoder":["",[["arch",[],["mod.rs","neon.rs","ssse3.rs"]],["decoder",[],["lossless.rs"]],["worker",[],["immediate.rs","mod.rs","multithreaded.rs","rayon.rs"]]],["decoder.rs","error.rs","huffman.rs","idct.rs","lib.rs","marker.rs","parser.rs","upsampler.rs"]],\
"lazy_static":["",[],["inline_lazy.rs","lib.rs"]],\
"lebe":["",[],["lib.rs"]],\
"libc":["",[["unix",[["linux_like",[["linux",[["arch",[["generic",[],["mod.rs"]]],["mod.rs"]],["gnu",[["b64",[["x86_64",[],["align.rs","mod.rs","not_x32.rs"]]],["mod.rs"]]],["align.rs","mod.rs"]]],["align.rs","mod.rs","non_exhaustive.rs"]]],["mod.rs"]]],["align.rs","mod.rs"]]],["fixed_width_ints.rs","lib.rs","macros.rs"]],\
"libloading":["",[["os",[["unix",[],["consts.rs","mod.rs"]]],["mod.rs"]]],["changelog.rs","error.rs","lib.rs","safe.rs","util.rs"]],\
"lock_api":["",[],["lib.rs","mutex.rs","remutex.rs","rwlock.rs"]],\
"log":["",[],["lib.rs","macros.rs"]],\
"matrixmultiply":["",[["x86",[],["macros.rs","mod.rs"]]],["aligned_alloc.rs","archparam_defaults.rs","debugmacros.rs","dgemm_kernel.rs","gemm.rs","kernel.rs","lib.rs","loopmacros.rs","ptr.rs","sgemm_kernel.rs","threading.rs","util.rs"]],\
"memchr":["",[["memchr",[["x86",[],["avx.rs","mod.rs","sse2.rs"]]],["fallback.rs","iter.rs","mod.rs","naive.rs"]],["memmem",[["prefilter",[["x86",[],["avx.rs","mod.rs","sse.rs"]]],["fallback.rs","genericsimd.rs","mod.rs"]],["x86",[],["avx.rs","mod.rs","sse.rs"]]],["byte_frequencies.rs","genericsimd.rs","mod.rs","rabinkarp.rs","rarebytes.rs","twoway.rs","util.rs","vector.rs"]]],["cow.rs","lib.rs"]],\
"memoffset":["",[],["lib.rs","offset_of.rs","raw_field.rs","span_of.rs"]],\
"miniz_oxide":["",[["deflate",[],["buffer.rs","core.rs","mod.rs","stream.rs"]],["inflate",[],["core.rs","mod.rs","output_buffer.rs","stream.rs"]]],["lib.rs","shared.rs"]],\
"nalgebra":["",[["base",[],["alias.rs","alias_slice.rs","allocator.rs","array_storage.rs","blas.rs","blas_uninit.rs","cg.rs","componentwise.rs","constraint.rs","construction.rs","construction_slice.rs","conversion.rs","coordinates.rs","default_allocator.rs","dimension.rs","edition.rs","helper.rs","indexing.rs","interpolation.rs","iter.rs","matrix.rs","matrix_simba.rs","matrix_slice.rs","min_max.rs","mod.rs","norm.rs","ops.rs","properties.rs","scalar.rs","statistics.rs","storage.rs","swizzle.rs","uninit.rs","unit.rs","vec_storage.rs"]],["geometry",[],["abstract_rotation.rs","dual_quaternion.rs","dual_quaternion_construction.rs","dual_quaternion_conversion.rs","dual_quaternion_ops.rs","isometry.rs","isometry_alias.rs","isometry_construction.rs","isometry_conversion.rs","isometry_interpolation.rs","isometry_ops.rs","isometry_simba.rs","mod.rs","op_macros.rs","orthographic.rs","perspective.rs","point.rs","point_alias.rs","point_construction.rs","point_conversion.rs","point_coordinates.rs","point_ops.rs","point_simba.rs","quaternion.rs","quaternion_construction.rs","quaternion_conversion.rs","quaternion_coordinates.rs","quaternion_ops.rs","quaternion_simba.rs","reflection.rs","reflection_alias.rs","rotation.rs","rotation_alias.rs","rotation_construction.rs","rotation_conversion.rs","rotation_interpolation.rs","rotation_ops.rs","rotation_simba.rs","rotation_specialization.rs","scale.rs","scale_alias.rs","scale_construction.rs","scale_conversion.rs","scale_coordinates.rs","scale_ops.rs","scale_simba.rs","similarity.rs","similarity_alias.rs","similarity_construction.rs","similarity_conversion.rs","similarity_ops.rs","similarity_simba.rs","swizzle.rs","transform.rs","transform_alias.rs","transform_construction.rs","transform_conversion.rs","transform_ops.rs","transform_simba.rs","translation.rs","translation_alias.rs","translation_construction.rs","translation_conversion.rs","translation_coordinates.rs","translation_ops.rs","translation_simba.rs","unit_complex.rs","unit_complex_construction.rs","unit_complex_conversion.rs","unit_complex_ops.rs","unit_complex_simba.rs"]],["linalg",[],["balancing.rs","bidiagonal.rs","cholesky.rs","col_piv_qr.rs","convolution.rs","decomposition.rs","determinant.rs","exp.rs","full_piv_lu.rs","givens.rs","hessenberg.rs","householder.rs","inverse.rs","lu.rs","mod.rs","permutation_sequence.rs","pow.rs","qr.rs","schur.rs","solve.rs","svd.rs","svd2.rs","svd3.rs","symmetric_eigen.rs","symmetric_tridiagonal.rs","udu.rs"]],["third_party",[["glam",[],["mod.rs"]]],["mod.rs"]]],["lib.rs"]],\
"nalgebra_macros":["",[],["lib.rs"]],\
"nanorand":["",[["crypto",[],["chacha.rs"]],["rand",[],["chacha.rs","pcg64.rs","wyrand.rs"]]],["buffer.rs","crypto.rs","entropy.rs","gen.rs","lib.rs","rand.rs","tls.rs"]],\
"num":["",[],["lib.rs"]],\
"num_bigint":["",[["bigint",[],["addition.rs","bits.rs","convert.rs","division.rs","multiplication.rs","power.rs","shift.rs","subtraction.rs"]],["biguint",[],["addition.rs","bits.rs","convert.rs","division.rs","iter.rs","monty.rs","multiplication.rs","power.rs","shift.rs","subtraction.rs"]]],["bigint.rs","biguint.rs","lib.rs","macros.rs"]],\
"num_complex":["",[],["cast.rs","complex_float.rs","lib.rs","pow.rs"]],\
"num_cpus":["",[],["lib.rs","linux.rs"]],\
"num_integer":["",[],["average.rs","lib.rs","roots.rs"]],\
"num_iter":["",[],["lib.rs"]],\
"num_rational":["",[],["lib.rs","pow.rs"]],\
"num_traits":["",[["ops",[],["checked.rs","euclid.rs","inv.rs","mod.rs","mul_add.rs","overflowing.rs","saturating.rs","wrapping.rs"]]],["bounds.rs","cast.rs","float.rs","identities.rs","int.rs","lib.rs","macros.rs","pow.rs","real.rs","sign.rs"]],\
"paste":["",[],["attr.rs","error.rs","lib.rs","segment.rs"]],\
"pin_project":["",[],["lib.rs"]],\
"pin_project_internal":["",[["pin_project",[],["args.rs","attribute.rs","derive.rs","mod.rs"]]],["lib.rs","pinned_drop.rs","utils.rs"]],\
"png":["",[["decoder",[],["mod.rs","stream.rs","zlib.rs"]]],["chunk.rs","common.rs","encoder.rs","filter.rs","lib.rs","srgb.rs","text_metadata.rs","traits.rs","utils.rs"]],\
"ppv_lite86":["",[["x86_64",[],["mod.rs","sse2.rs"]]],["lib.rs","soft.rs","types.rs"]],\
"proc_macro2":["",[],["detection.rs","fallback.rs","lib.rs","marker.rs","parse.rs","rcvec.rs","wrapper.rs"]],\
"proc_macro_error":["",[["imp",[],["fallback.rs"]]],["diagnostic.rs","dummy.rs","lib.rs","macros.rs","sealed.rs"]],\
"proc_macro_error_attr":["",[],["lib.rs","parse.rs","settings.rs"]],\
"quote":["",[],["ext.rs","format.rs","ident_fragment.rs","lib.rs","runtime.rs","spanned.rs","to_tokens.rs"]],\
"rand":["",[["distributions",[],["bernoulli.rs","distribution.rs","float.rs","integer.rs","mod.rs","other.rs","slice.rs","uniform.rs","utils.rs","weighted.rs","weighted_index.rs"]],["rngs",[["adapter",[],["mod.rs","read.rs","reseeding.rs"]]],["mock.rs","mod.rs","std.rs","thread.rs"]],["seq",[],["index.rs","mod.rs"]]],["lib.rs","prelude.rs","rng.rs"]],\
"rand_chacha":["",[],["chacha.rs","guts.rs","lib.rs"]],\
"rand_core":["",[],["block.rs","error.rs","impls.rs","le.rs","lib.rs","os.rs"]],\
"rawpointer":["",[],["lib.rs"]],\
"rayon":["",[["collections",[],["binary_heap.rs","btree_map.rs","btree_set.rs","hash_map.rs","hash_set.rs","linked_list.rs","mod.rs","vec_deque.rs"]],["compile_fail",[],["cannot_collect_filtermap_data.rs","cannot_zip_filtered_data.rs","cell_par_iter.rs","mod.rs","must_use.rs","no_send_par_iter.rs","rc_par_iter.rs"]],["iter",[["collect",[],["consumer.rs","mod.rs"]],["find_first_last",[],["mod.rs"]],["plumbing",[],["mod.rs"]]],["chain.rs","chunks.rs","cloned.rs","copied.rs","empty.rs","enumerate.rs","extend.rs","filter.rs","filter_map.rs","find.rs","flat_map.rs","flat_map_iter.rs","flatten.rs","flatten_iter.rs","fold.rs","for_each.rs","from_par_iter.rs","inspect.rs","interleave.rs","interleave_shortest.rs","intersperse.rs","len.rs","map.rs","map_with.rs","mod.rs","multizip.rs","noop.rs","once.rs","panic_fuse.rs","par_bridge.rs","positions.rs","product.rs","reduce.rs","repeat.rs","rev.rs","skip.rs","splitter.rs","step_by.rs","sum.rs","take.rs","try_fold.rs","try_reduce.rs","try_reduce_with.rs","unzip.rs","update.rs","while_some.rs","zip.rs","zip_eq.rs"]],["slice",[],["chunks.rs","mergesort.rs","mod.rs","quicksort.rs","rchunks.rs"]]],["array.rs","delegate.rs","lib.rs","math.rs","option.rs","par_either.rs","prelude.rs","private.rs","range.rs","range_inclusive.rs","result.rs","split_producer.rs","str.rs","string.rs","vec.rs"]],\
"rayon_core":["",[["compile_fail",[],["mod.rs","quicksort_race1.rs","quicksort_race2.rs","quicksort_race3.rs","rc_return.rs","rc_upvar.rs","scope_join_bad.rs"]],["join",[],["mod.rs"]],["scope",[],["mod.rs"]],["sleep",[],["counters.rs","mod.rs"]],["spawn",[],["mod.rs"]],["thread_pool",[],["mod.rs"]]],["job.rs","latch.rs","lib.rs","log.rs","private.rs","registry.rs","unwind.rs"]],\
"regex":["",[["literal",[],["imp.rs","mod.rs"]]],["backtrack.rs","compile.rs","dfa.rs","error.rs","exec.rs","expand.rs","find_byte.rs","input.rs","lib.rs","pikevm.rs","pool.rs","prog.rs","re_builder.rs","re_bytes.rs","re_set.rs","re_trait.rs","re_unicode.rs","sparse.rs","utf8.rs"]],\
"regex_syntax":["",[["ast",[],["mod.rs","parse.rs","print.rs","visitor.rs"]],["hir",[["literal",[],["mod.rs"]]],["interval.rs","mod.rs","print.rs","translate.rs","visitor.rs"]],["unicode_tables",[],["age.rs","case_folding_simple.rs","general_category.rs","grapheme_cluster_break.rs","mod.rs","perl_word.rs","property_bool.rs","property_names.rs","property_values.rs","script.rs","script_extension.rs","sentence_break.rs","word_break.rs"]]],["either.rs","error.rs","lib.rs","parser.rs","unicode.rs","utf8.rs"]],\
"rustversion":["",[],["attr.rs","bound.rs","constfn.rs","date.rs","error.rs","expand.rs","expr.rs","iter.rs","lib.rs","release.rs","time.rs","token.rs","version.rs"]],\
"safe_arch":["",[["x86_x64",[],["m128_.rs","m128d_.rs","m128i_.rs","m256_.rs","m256d_.rs","m256i_.rs","sse.rs","sse2.rs"]]],["lib.rs","naming_conventions.rs"]],\
"scoped_threadpool":["",[],["lib.rs"]],\
"scopeguard":["",[],["lib.rs"]],\
"semver":["",[],["lib.rs","version.rs","version_req.rs"]],\
"semver_parser":["",[],["common.rs","lib.rs","range.rs","recognize.rs","version.rs"]],\
"sim2d":["",[["application",[],["logging.rs","mod.rs","timer.rs"]],["graphics",[["renderer",[["texture",[],["mod.rs","texture_atlas.rs","texture_id.rs","texture_loader.rs"]]],["mod.rs"]],["vulkan_api",[["bindless_quads",[],["mod.rs","per_frame.rs","pipeline.rs"]],["command_buffer",[],["mod.rs"]],["frames_in_flight",[],["frame.rs","frame_sync.rs","mod.rs"]],["raii",[],["buffer.rs","command_pool.rs","descriptor_pool.rs","descriptor_set_layout.rs","image.rs","mod.rs","pipeline.rs","pipeline_layout.rs","shader_module.rs"]],["render_device",[],["mod.rs","queue.rs","queue_finder.rs","window_surface.rs"]],["render_pass",[],["color_pass.rs","mod.rs"]],["swapchain",[],["acquire_present.rs","mod.rs","selection.rs"]]],["mapped_buffer.rs","mod.rs","texture.rs"]]],["error.rs","mod.rs"]],["math",[],["mod.rs"]],["window",[["glfw_window",[],["mod.rs","window_state.rs"]]],["mod.rs"]]],["lib.rs","sim2d.rs","sketch.rs"]],\
"simba":["",[["scalar",[],["complex.rs","field.rs","mod.rs","real.rs","subset.rs"]],["simd",[],["auto_simd_impl.rs","mod.rs","simd_bool.rs","simd_complex.rs","simd_option.rs","simd_partial_ord.rs","simd_real.rs","simd_signed.rs","simd_value.rs","wide_simd_impl.rs"]]],["lib.rs"]],\
"smallvec":["",[],["lib.rs"]],\
"smawk":["",[],["lib.rs","monge.rs"]],\
"spin":["",[["mutex",[],["spin.rs"]]],["barrier.rs","lazy.rs","lib.rs","mutex.rs","once.rs","relax.rs","rwlock.rs"]],\
"spin_sleep":["",[],["lib.rs","loop_helper.rs"]],\
"syn":["",[["gen",[],["clone.rs","visit.rs","visit_mut.rs"]]],["attr.rs","await.rs","bigint.rs","buffer.rs","custom_keyword.rs","custom_punctuation.rs","data.rs","derive.rs","discouraged.rs","error.rs","export.rs","expr.rs","ext.rs","file.rs","gen_helper.rs","generics.rs","group.rs","ident.rs","item.rs","lib.rs","lifetime.rs","lit.rs","lookahead.rs","mac.rs","macros.rs","op.rs","parse.rs","parse_macro_input.rs","parse_quote.rs","pat.rs","path.rs","print.rs","punctuated.rs","reserved.rs","sealed.rs","span.rs","spanned.rs","stmt.rs","thread.rs","token.rs","ty.rs","verbatim.rs","whitespace.rs"]],\
"terminal_size":["",[],["lib.rs","unix.rs"]],\
"textwrap":["",[["core",[],["optimal_fit.rs"]]],["core.rs","indentation.rs","lib.rs","splitting.rs"]],\
"thiserror":["",[],["aserror.rs","display.rs","lib.rs"]],\
"thiserror_impl":["",[],["ast.rs","attr.rs","expand.rs","fmt.rs","generics.rs","lib.rs","prop.rs","valid.rs"]],\
"threadpool":["",[],["lib.rs"]],\
"tiff":["",[["decoder",[],["ifd.rs","image.rs","mod.rs","stream.rs","tag_reader.rs"]],["encoder",[["compression",[],["deflate.rs","lzw.rs","mod.rs","packbits.rs","uncompressed.rs"]]],["colortype.rs","mod.rs","tiff_value.rs","writer.rs"]]],["bytecast.rs","error.rs","lib.rs","tags.rs"]],\
"typenum":["",[],["array.rs","bit.rs","int.rs","lib.rs","marker_traits.rs","operator_aliases.rs","private.rs","type_operators.rs","uint.rs"]],\
"unicode_ident":["",[],["lib.rs","tables.rs"]],\
"unicode_width":["",[],["lib.rs","tables.rs"]],\
"vk_sys":["",[],["lib.rs"]],\
"weezl":["",[],["decode.rs","encode.rs","error.rs","lib.rs"]],\
"wide":["",[],["f32x4_.rs","f32x8_.rs","f64x2_.rs","f64x4_.rs","i16x16_.rs","i16x8_.rs","i32x4_.rs","i32x8_.rs","i64x2_.rs","i64x4_.rs","i8x16_.rs","i8x32_.rs","lib.rs","macros.rs","u16x8_.rs","u32x4_.rs","u32x8_.rs","u64x2_.rs","u64x4_.rs","u8x16_.rs"]]\
}');
createSourceSidebar();
