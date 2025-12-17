> [!Warning]
> Hic Sunt Dracones
>
> This project is in its infancy and this repository is far from being useful.

# raytracer
<p align="center"><img src="results/016-lights.png" width="500"></p>

A path tracer hobby project. Part of this project follows the book series *[Raytracing in One Weekend](https://raytracing.github.io)*.

## Examples
These are some examples of the current capabilities of this path tracer. Click on the images for full-size previews.

### Materials
Currently, the program supports diffuse and lambertian objects, as well as different metals and simple dielectrics like glass.
<p align="center">
 <img src="results/003-metal.png" width="400" style="display: inline-block">
 <img src="results/004-dielectric.png" width="400" style="display: inline-block">
</p>

### Camera Simulation
Different effects like depth of field and linear motion blur can be simulated.
<p align="center">
 <img src="results/005-depth-of-field.png" width="400" style="display: inline-block">
 <img src="results/006-motion-blur.png" width="400" style="display: inline-block">
</p>

### Environment Maps
HDRIs or environment maps can be used for scenes without light-emmitting objects. (The below images were produced with the same amount of samples, notice the noise difference between the brighter and darker environments.)
<p align="center">
 <img src="results/019-hdri-outside.png" width="400" style="display: inline-block">
 <img src="results/019-hdri-studio.png" width="400" style="display: inline-block">
</p>

### Textures
The raytracer currently supports some procedural textures (e.g. checker pattern and perlin noise) and image textures.
<p align="center">
 <img src="results/012-checker-texture.png" width="265" style="display: inline-block">
 <img src="results/014-perlin-noise.png" width="265" style="display: inline-block">
 <img src="results/013-image-texture.png" width="265" style="display: inline-block">
</p>

### Obligatory Cornell Box
To ensure the recognition by computer graphics researchers. Notice the immense amount of noise.
<p align="center"><img src="results/017-cornell-box.png" width="500"></p>

## License Notice
The texture at `/resources/earth.png` is licensed under CC BY-SA 3.0. Image by NASA Goddard Space Flight Center (Reto Stöckli, Robert Simmon). Data: MODIS Groups, USGS, DMSP.
