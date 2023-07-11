<div align="center">
 <a href="https://fontmeme.com/graffiti-creator/"><img src="https://fontmeme.com/permalink/230711/6010351cd75ecc1b080b911cf000c3f9.png" alt="graffiti-creator" border="0"></a>
</div>

<br>

<div align="center">


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
- My rust implementation of the book: 
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
