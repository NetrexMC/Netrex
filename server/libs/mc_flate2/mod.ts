// Auto-generated with deno_bindgen
import { CachePolicy, prepare } from "https://deno.land/x/plug@0.5.1/plug.ts"
function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v
  return new TextEncoder().encode(v)
}
function decode(v: Uint8Array): string {
  return new TextDecoder().decode(v)
}
function readPointer(v: any): Uint8Array {
  const ptr = new Deno.UnsafePointerView(v as Deno.UnsafePointer)
  const lengthBe = new Uint8Array(4)
  const view = new DataView(lengthBe.buffer)
  ptr.copyInto(lengthBe, 0)
  const buf = new Uint8Array(view.getUint32(0))
  ptr.copyInto(buf, 4)
  return buf
}
const opts = {
  name: "mc_flate2",
  url: (new URL("./bin/", import.meta.url)).toString(),
  policy: CachePolicy.NONE,
}
const _lib = await prepare(opts, {
  compress: {
    parameters: ["pointer", "usize"],
    result: "pointer",
    nonblocking: false,
  },
  decompress: {
    parameters: ["pointer", "usize"],
    result: "pointer",
    nonblocking: false,
  },
})

export function compress(a0: Uint8Array) {
  const a0_buf = encode(a0)
  let rawResult = _lib.symbols.compress(a0_buf, a0_buf.byteLength)
  const result = readPointer(rawResult)
  return result
}
export function decompress(a0: Uint8Array) {
  const a0_buf = encode(a0)
  let rawResult = _lib.symbols.decompress(a0_buf, a0_buf.byteLength)
  const result = readPointer(rawResult)
  return result
}