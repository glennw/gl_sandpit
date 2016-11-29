#ifdef VERTEX_SHADER
void main() {
    Rect rect = fetch_rect(gl_InstanceID, sFloat0);
    vec2 pos = mix(rect.p0,
                   rect.p1,
                   aPosition);

    vColor = vec4(1.0, 0.0, 0.0, 1.0);
    vUv = aPosition;

    gl_Position = vec4(uTransform.xy + pos * uTransform.zw, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT_SHADER
void main() {
    oFragColor = vec4(vUv, 0.0, 1.0);
}
#endif
