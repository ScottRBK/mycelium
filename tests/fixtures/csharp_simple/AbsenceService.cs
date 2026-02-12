using System;
using System.Collections.Generic;
using System.Linq;
using Absence.Models;
using Absence.Repositories;

namespace Absence.Services
{
    public class AbsenceService : IAbsenceService
    {
        private readonly IAbsenceRepository _repository;
        private readonly AbsenceModel _model;

        public AbsenceService(IAbsenceRepository repository)
        {
            _repository = repository;
            _model = new AbsenceModel();
        }

        public decimal CalculateEntitlement(int employeeId)
        {
            var baseEntitlement = _model.GetBaseEntitlement(employeeId);
            var bonusDays = GetBonusDays(employeeId);
            var taken = _repository.GetDaysTaken(employeeId);
            return baseEntitlement + bonusDays - taken;
        }

        private decimal GetBonusDays(int employeeId)
        {
            var yearsOfService = _repository.GetYearsOfService(employeeId);
            if (yearsOfService > 10) return 8.0m;
            if (yearsOfService > 5) return 5.0m;
            if (yearsOfService > 2) return 2.0m;
            return 0.0m;
        }

        public bool IsOnLeave(int employeeId)
        {
            var activeLeave = _repository.GetActiveLeave(employeeId);
            return activeLeave != null;
        }

        public List<AbsenceRecord> GetLeaveHistory(int employeeId, DateRange period)
        {
            return _repository.GetLeaveRecords(employeeId, period.StartDay, period.EndDay);
        }

        public LeaveType GetPrimaryLeaveType(int employeeId)
        {
            var records = _repository.GetLeaveRecords(employeeId, 0, int.MaxValue);
            if (records.Count == 0) return LeaveType.Annual;
            return records
                .GroupBy(r => r.Type)
                .OrderByDescending(g => g.Count())
                .First()
                .Key;
        }
    }
}
