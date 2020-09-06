// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

(INPUT)
   @color
   M=0        // color = 0
   @KBD
   D=M        // D = keyboard
   @PRESSED
   D;JNE      // if keyboard != 0 goto PRESSED
   @FILL
   0;JMP      // else goto FILL

(PRESSED)
   @color
   M=-1

(FILL)
   @SCREEN
   D=A
   @pixel
   M=D        // set pixel to the start address of SCREEN

   (LOOP)
      @color
      D=M
      @pixel
      A=M     // A = pixel
      M=D     // RAM[pixel] = color

      @pixel
      M=M+1   // move to the next pixel

      @24576  // one-past-the-end of SCREEN
      D=A
      @pixel
      D=D-M   // D = 24576 - pixel

      @LOOP
      D; JGT  // if 24576 - pixel > 0 goto LOOP

@INPUT
0;JMP
