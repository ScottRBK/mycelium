using System;
using MixedSolution.VBNet;

namespace MixedSolution.CSharp
{
    public class ApiController
    {
        private readonly DataProcessor _processor;

        public ApiController()
        {
            _processor = new DataProcessor();
        }

        public string HandleRequest(string input)
        {
            return _processor.Process(input);
        }
    }
}
