![banner](https://user-images.githubusercontent.com/49870539/133796956-0fe51a59-4e11-44a8-8ac7-b031b8944300.png)

# Calculator

A Calculator app made in rust that can ensure 100% accuracy.

# Features

* Has all the basic trigonometric functions (sin, cos, tan and arc versions)

* Uses the [pest parser](https://pest.rs/) to parse mathematical expressions containing +,-,/,* and ^

* Uses combined numbers to ensure that you can get the results of two numbers that can't be added (e.g. π and √2)

* Will warn the user by returning a double if the computation had to be rounded

* Quality of life features: automatic parenthesis balancing and insertion of * operator

# Screenshots

![multi-image](https://user-images.githubusercontent.com/49870539/133799957-3c494c1f-533f-4766-bfcf-83e024064a3f.png)
![multi-image](https://user-images.githubusercontent.com/49870539/133801154-0b2737e8-c6fd-474a-8f37-2e96c4ee17d2.png)

# Missing Features

- Add tests
- Add a nicer graphical interface
- Add complex numbers
- Allow equations to be typed like [MathJax](https://github.com/mathjax/MathJax-src)
