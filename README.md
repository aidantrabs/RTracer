<div align="center">

```
 _______  ________                                              
|       \|        \                                             
| ▓▓▓▓▓▓▓\\▓▓▓▓▓▓▓▓ ______   ______   _______  ______   ______  
| ▓▓__| ▓▓  | ▓▓   /      \ |      \ /       \/      \ /      \ 
| ▓▓    ▓▓  | ▓▓  |  ▓▓▓▓▓▓\ \▓▓▓▓▓▓\  ▓▓▓▓▓▓▓  ▓▓▓▓▓▓\  ▓▓▓▓▓▓\
| ▓▓▓▓▓▓▓\  | ▓▓  | ▓▓   \▓▓/      ▓▓ ▓▓     | ▓▓    ▓▓ ▓▓   \▓▓
| ▓▓  | ▓▓  | ▓▓  | ▓▓     |  ▓▓▓▓▓▓▓ ▓▓_____| ▓▓▓▓▓▓▓▓ ▓▓      
| ▓▓  | ▓▓  | ▓▓  | ▓▓      \▓▓    ▓▓\▓▓     \\▓▓     \ ▓▓      
 \▓▓   \▓▓   \▓▓   \▓▓       \▓▓▓▓▓▓▓ \▓▓▓▓▓▓▓ \▓▓▓▓▓▓▓\▓▓      

     
```

</div>

<br>

<div align="center">

![GitHub](https://img.shields.io/github/license/aidantrabs/RTracer?style=flat-square)

</div>

## Description :pushpin: 
- A lightweight CPU-based Rust raytracer that supports key features such as spheres, lambertian reflection, metal-like surfaces, and dielectrics to simulate light bouncing off and refracting through various materials. By tracing the path of light in reverse - from the camera back to the light source - the program calculates how light interacts with objects in a 3D scene to generate detailed, photorealistic images. It also uses anti-aliasing techniques for higher image quality and incorporates a pseudorandom number generator to model the random scattering of light and scene generation.

## Usage :hammer:
> Compile and redirect to PPM image format
```sh
$ cargo run > image.ppm
```

## Rendered Image :camera:
![Rendition](https://github.com/aidantrabs/RTracer/blob/main/output/image.png)

### References :paperclip:
- My Rust implementation of the book: 
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

### Next steps
- [ ] Implement: [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
