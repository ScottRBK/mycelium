using System.Collections.Generic;
using Absence.Models;

namespace Absence.Repositories
{
    public interface IAbsenceRepository
    {
        decimal GetDaysTaken(int employeeId);
        int GetYearsOfService(int employeeId);
        AbsenceRecord GetActiveLeave(int employeeId);
        List<AbsenceRecord> GetLeaveRecords(int employeeId, int fromDay, int toDay);
        void SaveLeaveRecord(AbsenceRecord record);
    }
}
