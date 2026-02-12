Imports System

Namespace Acme.Employee
    Public Class EmployeeService
        Private _repository As EmployeeRepository

        Public Sub New()
            _repository = New EmployeeRepository()
        End Sub

        Public Function GetEmployee(id As Integer) As String
            Return _repository.FindById(id)
        End Function

        Private Sub LogAccess(id As Integer)
            Console.WriteLine("Accessed " & id.ToString())
        End Sub
    End Class
End Namespace
