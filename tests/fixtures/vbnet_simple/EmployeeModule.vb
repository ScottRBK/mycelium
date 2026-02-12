Imports System

Namespace Acme.Employee
    Public Module EmployeeModule
        Public Sub LoadEmployee(employeeId As Integer)
            Dim svc As New EmployeeService()
            svc.GetEmployee(employeeId)
        End Sub

        Friend Sub ClearCache()
            ' internal helper
        End Sub
    End Module
End Namespace
