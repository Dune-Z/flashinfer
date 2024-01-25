#ifndef C_WRAPPER_CUH_
#define C_WRAPPER_CUH_
#include "flashinfer.cuh"
#include <vector>

#ifdef __cplusplus
extern "C" {
#endif
void vec_add(float *lhs, float *rhs, float *res, size_t size);

enum RotaryMode {
    kNone,
    kLlama,
}

enum QKVLayout {
}

cudaError_t singlePrefillWithKVCahceNoSLE(
    __half* q,
    __half* k,
    __half* v,
    __half* o,
    float* tmp,
    unsigned int num_qo_heads,
    unsigned int num_kv_heads,
    unsigned int qo_len,
    unsigned int kv_len,
    unsigned int head_dim,
    bool causal,
    QKVLayout layout,
    RotaryMode rotary_mode,
    bool allow_fp16_qk_reduction,
    float rope_scale,
    float rope_theta,
    cudaStream_t stream
);

#ifdef __cplusplus
}
#endif

#endif // C_WRAPPER_CUH_