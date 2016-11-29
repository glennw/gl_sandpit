#line 1

//======================================================================================
// Vertex shader attributes and uniforms
//======================================================================================
#ifdef VERTEX_SHADER
    #define MAX_VERTEX_TEXTURE_WIDTH 1024
    #define varying out

    // Uniforms
    uniform vec4 uTransform;
    uniform sampler2D sFloat0;
    uniform sampler2D sFloat1;

    // Attribute inputs
    in vec2 aPosition;
#endif

//======================================================================================
// Fragment shader attributes and uniforms
//======================================================================================
#ifdef FRAGMENT_SHADER
    precision highp float;

    #define varying in

    // Uniforms

    // Fragment shader outputs
    out vec4 oFragColor;
#endif

//======================================================================================
// Interpolator definitions
//======================================================================================
varying vec4 vColor;
varying vec2 vUv;

//======================================================================================
// VS only types and UBOs
//======================================================================================

//======================================================================================
// VS only functions
//======================================================================================
#ifdef VERTEX_SHADER

struct Rect {
    vec2 p0;
    vec2 p1;
};

ivec2 get_fetch_uv(int index, int vecs_per_item) {
    int items_per_row = MAX_VERTEX_TEXTURE_WIDTH / vecs_per_item;
    int y = index / items_per_row;
    int x = vecs_per_item * (index % items_per_row);
    return ivec2(x, y);
}

vec4 fetch_vec4(int index, sampler2D s) {
    ivec2 uv = get_fetch_uv(index, 1);
    return texelFetchOffset(s, uv, 0, ivec2(0, 0));
}

Rect fetch_rect(int index, sampler2D s) {
    ivec2 uv = get_fetch_uv(index, 1);

    vec4 data = texelFetchOffset(s, uv, 0, ivec2(0, 0));

    Rect rect;
    rect.p0 = data.xy;
    rect.p1 = rect.p0 + data.zw;

    return rect;
}

#endif
