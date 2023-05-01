import macros

macro generate_sort_tests*(sort_name: static[string]): untyped =
  result = newStmtList()
  let sort_proc = parseExpr(sort_name)
  let tests = @[
    ("One element list", @[5], @[5]),
    ("Sorted list", @[1, 2, 3, 4, 5], @[1, 2, 3, 4, 5]),
    ("Reverse sorted list", @[5, 4, 3, 2, 1], @[1, 2, 3, 4, 5]),
    ("Unsorted list", @[3, 5, 1, 4, 2], @[1, 2, 3, 4, 5]),
    ("List with duplicates", @[3, 5, 1, 4, 2, 3], @[1, 2, 3, 3, 4, 5])
    ]
  
  for test in tests:
    let testName = test[0]
    let inputList = test[1]
    let expectedList = test[2]
    
    let testBody = quote do:
      var elements = `inputList`
      `sort_proc`(elements)
      doAssert elements == `expectedList`
    
    result.add(newNimNode(nnkCommand).add(newIdentNode("test"), newStrLitNode(testName), testBody))