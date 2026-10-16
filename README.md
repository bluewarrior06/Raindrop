![alt text](https://github.com/bluewarrior06/Raindrop/blob/main/repo_assets/icon_with_text.png "Image")

* Small developer notice:
**Currently, Raindrop is experimental. I am unsure on if this project will be finished, or abandoned. My hope is that I make something functional, because this is a really fun concept for me to develop currently!**

# What is Raindrop?
Raindrop's main focus is making procedurally generated images, similar to how level graphics are created in Rain World. You color in some tiles, add some props, then generate an image!

This is currently JUST a procedural image generator. There are no level creation features, or tile data that can be generated and exported. **Though**, the idea of transforming this into a sort of semi general purpose level editor would definitely be touched upon, since this is used to generate level-like images.

# Concepts
## Painters, Layers and Effects
The main core of the image generation comes from a painter. Each painter holds a layer and an effect. 

The layer holds the information, such as color values in a grid, or the placement of props. 

The effect reads information from a layer, and uses that to draw.

There are two different types of painters, layers, and effects:
* Grid - Layer data is stored as a fixed-sized grid.
* Prop - Layer data is stored as a non fixed-sized list. 

## Images & Output
Effects draw onto what is known as an "output". The output holds a number of images.
An image is just a bitmap 

These images are:
* Albedo - The 
* light
