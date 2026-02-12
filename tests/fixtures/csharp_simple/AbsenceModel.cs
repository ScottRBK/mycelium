using System;

namespace Absence.Models
{
    internal class AbsenceModel
    {
        private const decimal DefaultEntitlement = 25.0m;
        private const decimal MaxEntitlement = 40.0m;

        public decimal GetBaseEntitlement(int employeeId)
        {
            return DefaultEntitlement;
        }

        public decimal ClampEntitlement(decimal value)
        {
            return Math.Min(value, MaxEntitlement);
        }

        protected void Reset()
        {
            // Reset internal calculation state
        }
    }

    public enum LeaveType
    {
        Annual,
        Sick,
        Maternity,
        Paternity,
        Unpaid,
        Compassionate
    }

    public struct DateRange
    {
        public int StartDay;
        public int EndDay;

        public int Duration => EndDay - StartDay;
    }

    public class AbsenceRecord
    {
        public int Id { get; set; }
        public int EmployeeId { get; set; }
        public LeaveType Type { get; set; }
        public DateRange Period { get; set; }
        public decimal Days { get; set; }
        public bool Approved { get; set; }
    }

    public class LeaveRequest
    {
        public int EmployeeId { get; set; }
        public LeaveType Type { get; set; }
        public DateRange Period { get; set; }
        public decimal Days { get; set; }
        public string Reason { get; set; }
    }

    public class LeaveResponse
    {
        public string RequestId { get; set; }
        public bool Approved { get; set; }
        public decimal RemainingEntitlement { get; set; }
    }
}
