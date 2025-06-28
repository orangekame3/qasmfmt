Output img/demo.gif
Set Margin 24
Set MarginFill "#674EFF"
Set BorderRadius 20

Set FontSize 32
Set Width 1500
Set Height 1000
Sleep 500ms

Type "cat testdata/test_unformatted.qasm"
Enter
Sleep 1s

Type "qasmfmt testdata/test_unformatted.qasm"
Enter
Sleep 3s

Type "echo 'OPENQASM 3.0;qubit[2]q;hq[0];' | qasmfmt format"
Enter
Sleep 3s

Type "qasmfmt -w testdata/test_unformatted.qasm"
Enter
Sleep 3s

Type "cat testdata/test_unformatted.qasm"
Enter
Sleep 3s
