# An Image editor written in rust

the editor start with Canvas struct, it takes a resolution and hold an array of layers

## Layer
Layer is the main building block inside the editor, it functions like a photoshop layer, it carries id (so you can edit it through canvas), Element, position, scale and adjustments List

so you create the canvas and then create a layer (either through providing an Image or graphic), this process return and Id that you can use later to edit the layer

## Layer ID
you can edit layer by getting a mutable ref to it from canvas by using the id

## Element
It is the actual image/graphic inside the layer, you can add graphic directly through their data (like width, color, stroke width, points in case of a line), if you want to add an image you need to import it first through the image struct

## Position
Each layer has a position field that carry position for x and y each one independently, Position can be a (point, percentage, center, end, start), they are only set, so there is no function to add on existing position

## Adjustments List
Adjustments are stored in the layer struct, they are applied on render only, so the original image is preserved, we have adjustments like (exposure, saturation, Brightness, Rotatation left or right (only 90 degree for now), mirroring vertically or horizontally)

## Layering
Layers are ordered through there order inside the layers array inside canvas, last is the uppermost, you can change their order through functions (bring_layer_to_front, bring_layer_forward, send_layer_backward, send_to_back), those processes doesn't affect the id, although the ID might be equal to the layer index initially

## Graphics
We draw graphics through storing their metadata like start point and end point for line and start point and width/height for rectangle, they AREN'T treated as raster images
- For now scale and position properties DOESN'T work on Graphics Layer although they exist and can be changed

## Image encoding/decoding
We can render png and bmp formats only for now, we can encode alpha channel if it exists in the image, we mainly use the bottom to top left to write in pixel arrangment, so it works natively with bmp but required some altering with png

## Pixel
Pixel is a struct that contain red, green, blue and alpha fields, alpha is 255 when it doesn't exist in the image

## Exporting
The only exporting option for now is transparent which is a bool for having alpha channel
