Monte Carlo Ray Tracer written in Rust from scratch. It features:
* Multithreading
* Extremely fast Quad-BVH (Bounding Volume Hierarchy) with SIMD
* Vast material selection (Lambertian, textured, metal, colored dielectric, isotropic volume (fog or smoke), Blinn-Phong, anisotropic Ashikhmin-Shirley)
* .obj loader
* Triangle Rendering
* HDRI background
* Light Sampling
* Bloom effect

![image-042](https://github.com/miguelggcc/raytracer/assets/100235899/855d7bf4-f269-4494-b6da-a60e0845e6dfc|width=320px)
![image-056](https://github.com/miguelggcc/raytracer/assets/100235899/6282162e-1635-43fb-a8ef-fed68a441835|width=320px)
![image-064](https://github.com/miguelggcc/raytracer/assets/100235899/6e131e57-a1b1-4f97-bbd3-514eca7e5ccf|width=320px)
![image-070_intel](https://github.com/miguelggcc/raytracer/assets/100235899/2010969b-3841-4551-abd2-a82f95d23ffa|width=320px)
![image-076_intel](https://github.com/miguelggcc/raytracer/assets/100235899/d63cc9bd-37f3-42f5-a664-60a6089ea59c|width=320px)



[1] Shirley, P. (2018-2020). *Ray Tracing in One Weekend Book Series*. GitHub. Retrieved from [https://github.com/RayTracing/raytracing.github.io](https://github.com/RayTracing/raytracing.github.io)

[2] Pharr, M., Jakob, W., & Humphreys, G. (2023). *Physically Based Rendering: From Theory to Implementation* (4th ed.). MIT Press. Retrieved from [https://www.pbrt.org/](https://www.pbrt.org/)

[3] Shirley, P., & Morley, R. K. (2003). *Realistic Ray Tracing* (2nd ed.). A K Peters/CRC Press.

[4] Dammertz, H., Hanika, J., & Keller, A. (2008). *Shallow Bounding Volume Hierarchies for Fast SIMD Ray Tracing of Incoherent Rays*. Computer Graphics Forum.
