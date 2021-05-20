<div align="center">
     <p>
          <img width="150" alt="Netrex" src="https://i.imgur.com/I1unWMx.png">
     </p>
     <p>
          <h1> Netrex </h1>
          <p>A powerful minecraft bedrock software written in Typescript with Deno.</p>
     </p>
</div>

## Why Netrex?
 - It's written in Typescript.
 - Unique and straight to the point.
 - Single Executable thanks to Deno.
 - Open Source

## The Game Plan

We have a plan for the initial release! View it on our [projects page](https://github.com/NetrexMC/Netrex-den/projects/1)! On top of this, we are working on a closed rust fork with the same api so Netrex will be fast in the future.

## Scripts
All scripts can be accessed and viewed through the terminal by running `deno run -A ./scripts/mod.ts` in the home directory, or by installing them with deno. In this case, the name will be "netrexscript".

### Install Netrex Scripts

```bash
deno install -A -f -n netrexscript https://raw.githubusercontent.com/NetrexMC/Netrex-den/master/scripts/mod.ts
```

### Count lines

A simple utility that recursively counts all lines in a given directory regardless of the file type (which will be changed later).

```bash
netrexscript count ./
```

### Apply Headers

A simple utility that applies the netrex headers to all files within the given directory.

```bash
netrexscript headers ./
```

