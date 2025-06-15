import macros, bitops

type Result[T, E] = ref object
  case ok: bool
  of true:
    value: T
  of false:
    error: E

proc makeOk[T, E](value: T): Result[T, E] =
  return Result[T, E](ok: true, value: value)

proc makeErr[T, E](error: E): Result[T, E] =
  return Result[T, E](ok: false, error: error)

macro exportwasm*(args: varargs[untyped]): untyped =
  var
    p: NimNode
    name: string
  if args.len > 1:
    p = args[1]
    name = $args[0]
  else:
    p = args[0]
    name = $args[0].name

  let codegenPragma = "__attribute__ ((export_name (\"" & name & "\"))) $# $#$#"
  result = p
  result.addPragma(newColonExpr(ident"codegenDecl", newLit(codegenPragma)))
  result.addPragma(ident"exportc")

proc run(): int32 {.exportwasm: "wasi:cli/run@0.2.0#run".} =
  let a = 5
  let b = 10
  let c = a + b

  let r =
    if c == 15:
      makeOk[string, string]("c is 15")
    else:
      makeErr[string, string]("c is not 15")

  return cast[int32](0)
