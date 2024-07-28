import "a"
import {x} from "a"
import y from "a"
import * as z from "a"
import {x as y} from "a"
import y, {x} from "a"
import y, * as z from "a"
import "a" assert { x: 1 }
import {x} from "a" assert { x: 1 }
export {x}
export const x = 1
export function f () {}
export default function () {}
export * from "a"
export {z} from "a"
export {x as y} from "a"
