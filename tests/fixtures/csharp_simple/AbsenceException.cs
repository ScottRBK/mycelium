using System;

namespace Absence.Models
{
    public class AbsenceException : Exception
    {
        public string ErrorCode { get; }

        public AbsenceException(string message) : base(message)
        {
            ErrorCode = "ABSENCE_ERROR";
        }

        public AbsenceException(string message, string errorCode) : base(message)
        {
            ErrorCode = errorCode;
        }

        public AbsenceException(string message, Exception innerException)
            : base(message, innerException)
        {
            ErrorCode = "ABSENCE_ERROR";
        }
    }
}
