# CAT TRANSLATOR

## how to use

to get a list of commands and their description:

```bash
cat_translator --help
```

or

```bash
cat_translator -h
```

to encode text into cat noises:

```bash
cat_translator crypt "your text"
```

to decode cat noises into text

```bash
cat_translator decrypt "your cat noises"
```

to encode or decode multiple times you can use the -d flag putting the depth afterward (if you want to crypt it two times -d 2)
```bash
cat_translator crypt -d 2 ":3"
cat_translator decrypt -d 2 "meoww purrrr mroww meow mrp mrrp meoww meowww meowww mrrrrp purr mrowww meow mrow meoww purr mroww mrrp mrowwww mrp purr meowww meowww meowwww mrp mrrrrp meoww mrowww meowww purr mroww mrrp mrowwww purrrr mrp meowwww mrp mrrrrp purrrrr mrow meoww purr"
```

## CREDITS
I coded the rust cli alone but the whole cat noises encoding system has been designed by Freya (thenonymous),
the license of this repo only applies to the rust code I guess then
