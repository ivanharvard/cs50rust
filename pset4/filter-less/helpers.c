#include "helpers.h"

void grayscale_rs(int height, int width, RGBTRIPLE image[height][width]);
void sepia_rs(int height, int width, RGBTRIPLE image[height][width]);
void reflect_rs(int height, int width, RGBTRIPLE image[height][width]);
void blur_rs(int height, int width, RGBTRIPLE image[height][width]);

// Convert image to grayscale
void grayscale(int height, int width, RGBTRIPLE image[height][width])
{
    grayscale_rs(height, width, image);
}

// Convert image to sepia
void sepia(int height, int width, RGBTRIPLE image[height][width])
{
    sepia_rs(height, width, image);
}

// Reflect image horizontally
void reflect(int height, int width, RGBTRIPLE image[height][width])
{
    reflect_rs(height, width, image);
}

// Blur image
void blur(int height, int width, RGBTRIPLE image[height][width])
{
    blur_rs(height, width, image);
}
