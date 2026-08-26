# CAT TRANSLATOR

## how to install

```bash
cargo install cat_translator
```

[see crates.io package](https://crates.io/crates/cat_translator)

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

cat_translator supports reading from the standard input for both crypt and decrypt e.g.

```bash
echo "meow" | cat_translator crypt
echo "meowww mrowww meoww meow mrrp meoww meowwww" | cat_translator decrypt
```

to encode or decode multiple times you can use the -d flag putting the depth afterward (if you want to crypt it two times -d 2)
```bash
cat_translator crypt -d 2 ":3"
cat_translator decrypt -d 2 "meoww purrrr mroww meow mrp mrrp meoww meowww meowww mrrrrp purr mrowww meow mrow meoww purr mroww mrrp mrowwww mrp purr meowww meowww meowwww mrp mrrrrp meoww mrowww meowww purr mroww mrrp mrowwww purrrr mrp meowwww mrp mrrrrp purrrrr mrow meoww purr"
```

to encode cat noises into its binary representation and vice-versa

```bash
cat_translator crypt "meow" --to-bin
cat_translator crypt "0001100000010000011100010110" --from-bin
```

to decode cat noises into its binary representation and vice-versa

```bash
cat_translator decrypt "meoww mrp meoww meow mrowwww meoww mrowww" --to-bin
cat_translator decrypt "0001100000010000011100010110" --from-bin
```

## CREDITS
I coded the rust cli alone but the whole cat noises encoding system has been designed by Freya (thenonymous),
the license of this repo only applies to the rust code I guess then
