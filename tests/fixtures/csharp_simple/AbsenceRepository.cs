using System.Collections.Generic;
using System.Linq;
using Absence.Models;

namespace Absence.Repositories
{
    public class AbsenceRepository : IAbsenceRepository
    {
        private readonly List<AbsenceRecord> _records = new List<AbsenceRecord>();
        private readonly Dictionary<int, int> _yearsOfService = new Dictionary<int, int>();

        public decimal GetDaysTaken(int employeeId)
        {
            return _records
                .Where(r => r.EmployeeId == employeeId && r.Approved)
                .Sum(r => r.Days);
        }

        public int GetYearsOfService(int employeeId)
        {
            return _yearsOfService.TryGetValue(employeeId, out var years) ? years : 0;
        }

        public AbsenceRecord GetActiveLeave(int employeeId)
        {
            return _records.FirstOrDefault(r =>
                r.EmployeeId == employeeId &&
                r.Approved &&
                r.Period.StartDay <= CurrentDay() &&
                r.Period.EndDay >= CurrentDay());
        }

        public List<AbsenceRecord> GetLeaveRecords(int employeeId, int fromDay, int toDay)
        {
            return _records
                .Where(r => r.EmployeeId == employeeId &&
                            r.Period.StartDay >= fromDay &&
                            r.Period.EndDay <= toDay)
                .OrderBy(r => r.Period.StartDay)
                .ToList();
        }

        public void SaveLeaveRecord(AbsenceRecord record)
        {
            record.Id = _records.Count + 1;
            _records.Add(record);
        }

        private int CurrentDay()
        {
            return 0; // Simplified for demo
        }
    }
}
