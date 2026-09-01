/*
 * Thin wrapper so bindgen can pull in the real VPP stats-segment client
 * API. Unlike vpp-api-transport's shmem_wrapper.h (a minimal hand-written
 * subset with no VPP header dependencies), stat_client.h transitively
 * includes real VPP headers (vlib/counter_types.h, vlib/stats/shared.h)
 * and uses their real types (stat_directory_type_t, counter_t,
 * vlib_counter_t, vlib_stats_shared_header_t, ...). Do not hand-flatten
 * those types here - point bindgen's clang args at a real VPP include
 * directory (see build.rs's VPP_INCLUDE_DIR) so it resolves the actual
 * headers, the same way a C compiler building a real VPP plugin would.
 */
#include <vpp-api/client/stat_client.h>
