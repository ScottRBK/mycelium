Imports System
Imports Acme.Employee

Namespace Acme.Utils
    Public Module EmployeeUtils
        Public Function FormatEmployeeName(firstName As String, lastName As String) As String
            Return UCase(firstName) & " " & UCase(lastName)
        End Function

        Public Function CalculateAge(birthYear As Integer) As Integer
            Return DateTime.Now.Year - birthYear
        End Function

        Public Sub LogEmployeeAction(employeeId As Integer, action As String)
            Console.WriteLine("Employee " & CStr(employeeId) & ": " & action)
        End Sub

        Friend Function ValidateEmployeeId(id As Integer) As Boolean
            Return id > 0
        End Function

        Private Sub InternalHelper()
            Call LogEmployeeAction(0, "internal")
        End Sub
    End Module
End Namespace
