> For the complete documentation index, see [llms.txt](/llms.txt)

# Compact formatter usage page

This is the usage page of **format-compact**, a formatter for Compact. This document assumes you are using **format-compact** directly. Typically, however, the formatter is invoked via the **compact** tool's `format` command, which is described in the [usage page for that tool](/compact/compilation-and-tooling/dev-tool-usage.md).

# NAME

format-compact

# OVERVIEW

The Compact formatter is part of the Compact toolchain. It takes as input a Compact source program in a specified source file, reformats it, and writes the reformatted program to a specified file. If such a file is not specified, it writes the reformatted program to standard output.

# SYNOPSIS

**format-compact** *flag* **...** *sourcepath* *targetpath*

# DESCRIPTION

The flags *flag* **...** are optional. They are described under FLAGS later in this document.

*sourcepath* should specify a file containing a Compact source program, and *targetpath* should specify a target file in which the reformatted program is to be written. *targetpath* may be an existing file, in which case the file will be replaced with the formatted program. *targetpath* may be the same as *sourcepath*, in which case the source program is replaced with the reformatted equivalent.

# FLAGS

The following flags, if present, affect the formatter's behavior as follows:

**--help**

Prints help text and exits.

**--version**

Prints the compiler version and exits.

**--language-version**

Prints the language version and exits.

**--vscode**

Causes error messages to be printed on a single line so they are rendered properly within the VS Code extension for Compact.

**--line-length *n***

Sets the target line length to *n* (default 100).

# EXAMPLES

Assuming `src/test.compact` contains a well-formed Compact program, running:

```
format-compact src/test.compact
```

prints the formatted program of **src/test.compact** to standard output.

Assuming that **formatted** is an existing directory, running:

```
format-compact src/test.compact formatted/test.compact
```

writes the formatted program to **formatted/test.compact**. If the **formatted** directory does not exist the formatter complains that it cannot create the output file.

Alternatively, running:

```
format-compact src/test.compact src/test.compact
```

rewrites the formatted program to **src/test.compact**.

Assuming **src/test.compact** contains an ill-formed Compact program, running:

```
format-compact src/test.compact
```

exits with an error message describing the problem that prevents the Compact program in **src/test.compact** from compiling.
