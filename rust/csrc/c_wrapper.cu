#include "c_wrapper.cuh"
#include <iostream>

extern "C" {
void vec_add(float *lhs, float *rhs, float *res, size_t size) {
    for (size_t i = 0; i < size; ++i) {
        res[i] = lhs[i] + rhs[i];
    }
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
    QKVLayout layout = QKVLayout::kNHD,
    RotaryMode rotary_mode = RotaryMode::kNone,
    bool allow_fp16_qk_reduction = false,
    float rope_scale = 1.f, float rope_theta = 1e4,
    cudaStream_t stream = nullptr
) {
}
}