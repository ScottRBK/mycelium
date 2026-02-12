using System;
using Absence.Models;

namespace Absence.Validators
{
    public class LeaveRequestValidator
    {
        private const decimal MaxRequestDays = 30.0m;
        private const int MinNoticeDays = 5;

        public void ValidateRequest(LeaveRequest request)
        {
            if (request == null)
                throw new AbsenceException("Request cannot be null");

            if (request.Days <= 0)
                throw new AbsenceException("Days must be positive");

            if (request.Days > MaxRequestDays)
                throw new AbsenceException($"Cannot request more than {MaxRequestDays} days");

            if (request.Period.Duration < 0)
                throw new AbsenceException("Invalid date range");

            ValidateLeaveType(request.Type);
        }

        private void ValidateLeaveType(LeaveType type)
        {
            if (type == LeaveType.Unpaid)
            {
                // Unpaid leave requires manager approval
            }
        }
    }
}
