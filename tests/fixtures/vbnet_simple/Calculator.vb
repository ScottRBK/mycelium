Imports System
Imports System.Collections.Generic
Imports System.Threading.Tasks

Namespace MyApp.Calculator

    Public Delegate Sub OperationCompleted(result As Double)

    Public Enum OperationType
        Add
        Subtract
        Multiply
        Divide
    End Enum

    Public Interface ICalculator
        Function Calculate(a As Double, b As Double, op As OperationType) As Double
        Sub Reset()
    End Interface

    Public Structure CalculationResult
        Public Value As Double
        Public Operation As OperationType
        Public Timestamp As DateTime
    End Structure

    Public Class Calculator
        Private _lastResult As Double

        Public Sub New()
            _lastResult = 0
        End Sub

        Public Function Calculate(a As Double, b As Double, op As OperationType) As Double
            Dim result As Double = PerformOperation(a, b, op)
            LogResult(result, op)
            Return result
        End Function

        Public Sub Reset()
            _lastResult = 0
        End Sub

        Public Property LastResult As Double
            Get
                Return _lastResult
            End Get
            Private Set(value As Double)
                _lastResult = value
            End Set
        End Property

        Private Function PerformOperation(a As Double, b As Double, op As OperationType) As Double
            Select Case op
                Case OperationType.Add
                    Return a + b
                Case OperationType.Subtract
                    Return a - b
                Case OperationType.Multiply
                    Return a * b
                Case OperationType.Divide
                    Return a / b
                Case Else
                    Return 0
            End Select
        End Function

        Private Sub LogResult(result As Double, op As OperationType)
            _lastResult = result
        End Sub

        Protected Function GetHistory() As Integer
            Return 0
        End Function

        Friend Sub ClearHistory()
            _lastResult = 0
        End Sub
    End Class

    Public Module MathHelpers
        Public Function Square(x As Double) As Double
            Return x * x
        End Function

        Public Function Cube(x As Double) As Double
            Return x * x * x
        End Function

        Private Function Clamp(value As Double, min As Double, max As Double) As Double
            Return value
        End Function
    End Module

End Namespace
