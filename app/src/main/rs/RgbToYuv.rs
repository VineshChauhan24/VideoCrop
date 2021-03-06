#pragma version(1)
#pragma rs_fp_relaxed
#pragma rs java_package_name(ru.cherryperry.instavideo.data.media.renderscript)

rs_allocation yuv;
ushort width;
ushort height;
ushort type;

void RS_KERNEL convert(uchar4 v_in, uint32_t x, uint32_t y) {
    uint3 yuv_v;
    yuv_v.s0 = clamp(((66 * v_in.r + 129 * v_in.g + 25 * v_in.b + 128) >> 8) + 16, 0, 255);
    yuv_v.s1 = clamp(((-38 * v_in.r - 74 * v_in.g + 112 * v_in.b + 128) >> 8) + 128, 0, 255);
    yuv_v.s2 = clamp(((112 * v_in.r - 94 * v_in.g - 18 * v_in.b + 128) >> 8) + 128, 0, 255);

    uint3 yuv_p;
    if (type == 0) {
        // YUV420P
        yuv_p.s0 = y * width + x;
        yuv_p.s1 = y / 2 * width / 2 + x / 2 + width * height;
        yuv_p.s2 = y / 2 * width / 2 + x / 2 + width * height + width * height / 4;
    }
    if (type == 1) {
        // TODO Packed
        yuv_p.s0 = y * width + x;
        yuv_p.s1 = yuv_p.s0;
        yuv_p.s2 = yuv_p.s0;
    }
    if (type == 2) {
        // YUV420SP NV12
        yuv_p.s0 = y * width + x;
        yuv_p.s1 = width * height + y / 2 * width + (x & ~1);
        yuv_p.s2 = yuv_p.s1 + 1;
    }
    if (type == 3) {
        // YUV420SP NV21
        yuv_p.s0 = y * width + x;
        yuv_p.s2 = width * height + y / 2 * width + (x & ~1);
        yuv_p.s1 = yuv_p.s2 + 1;
    }

    rsSetElementAt_uchar(yuv, yuv_v.s0, yuv_p.s0);

    if (x % 2 == 0 && y % 2 == 0) {
        rsSetElementAt_uchar(yuv, yuv_v.s1, yuv_p.s1);
        rsSetElementAt_uchar(yuv, yuv_v.s2, yuv_p.s2);
    }
}
