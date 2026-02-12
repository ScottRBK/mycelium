Imports System
Imports System.Collections.Generic
Imports Acme.Employee

Namespace Acme.Repositories
    Public Interface IEmployeeRepository
        Function FindById(id As Integer) As String
        Function FindAll() As List(Of String)
        Sub Save(name As String)
        Sub Delete(id As Integer)
        Function GetDaysTaken(employeeId As Integer) As Decimal
    End Interface

    Public Class EmployeeRepository
        Implements IEmployeeRepository

        Private _data As New Dictionary(Of Integer, String)

        Public Sub New()
        End Sub

        Public Function FindById(id As Integer) As String Implements IEmployeeRepository.FindById
            If _data.ContainsKey(id) Then
                Return _data(id)
            End If
            Return Nothing
        End Function

        Public Function FindAll() As List(Of String) Implements IEmployeeRepository.FindAll
            Return New List(Of String)(_data.Values)
        End Function

        Public Sub Save(name As String) Implements IEmployeeRepository.Save
            Dim nextId As Integer = _data.Count + 1
            _data.Add(nextId, name)
        End Sub

        Public Sub Delete(id As Integer) Implements IEmployeeRepository.Delete
            _data.Remove(id)
        End Sub

        Public Function GetDaysTaken(employeeId As Integer) As Decimal Implements IEmployeeRepository.GetDaysTaken
            Return 5.0D
        End Function

        Private Function FormatName(name As String) As String
            Return UCase(Trim(name))
        End Function
    End Class
End Namespace
