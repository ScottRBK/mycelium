using System;
using System.Collections.Generic;
using Absence.Services;
using Absence.Models;
using Absence.Validators;

namespace Absence.Controllers
{
    public class AbsenceController
    {
        private readonly IAbsenceService _service;
        private readonly LeaveRequestValidator _validator;

        public AbsenceController(IAbsenceService service, LeaveRequestValidator validator)
        {
            _service = service;
            _validator = validator;
        }

        public decimal GetEntitlement(int employeeId)
        {
            var result = _service.CalculateEntitlement(employeeId);
            return result;
        }

        public bool CheckLeaveStatus(int employeeId)
        {
            return _service.IsOnLeave(employeeId);
        }

        public LeaveResponse SubmitRequest(LeaveRequest request)
        {
            _validator.ValidateRequest(request);
            var remaining = _service.CalculateEntitlement(request.EmployeeId);
            var approved = remaining >= request.Days;
            return new LeaveResponse
            {
                RequestId = Guid.NewGuid().ToString(),
                Approved = approved,
                RemainingEntitlement = remaining - (approved ? request.Days : 0)
            };
        }

        public List<AbsenceRecord> GetLeaveHistory(int employeeId, DateRange period)
        {
            return _service.GetLeaveHistory(employeeId, period);
        }

        internal void RefreshCache()
        {
            // Invalidate cached entitlement calculations
        }
    }
}
