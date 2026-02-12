using System.Collections.Generic;
using Absence.Models;

namespace Absence.Services
{
    public interface IAbsenceService
    {
        decimal CalculateEntitlement(int employeeId);
        bool IsOnLeave(int employeeId);
        List<AbsenceRecord> GetLeaveHistory(int employeeId, DateRange period);
        LeaveType GetPrimaryLeaveType(int employeeId);
    }
}
