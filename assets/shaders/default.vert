void main()
{
    // transform the vertex position
    gl_Position = gl_ModelViewProjectionMatrix * gl_Vertex;

    // transform the texture coordinates
    // gl_TextureMatrix[0] allows for pixel-space tex coords
    gl_TexCoord[0] = gl_TextureMatrix[0] * gl_MultiTexCoord0;
    //gl_TexCoord[0] = gl_MultiTexCoord0;

    // forward the vertex color
    gl_FrontColor = gl_Color;
}