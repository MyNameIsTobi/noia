using System.Diagnostics;
using System.Linq;
using noia.ViewModels;

namespace noia.Helpers
{
    public static class ProcessHelper
    {
        /// <summary>
        /// Retrieves all processes whose ProcessName is "aion".
        /// (The actual file is "aion.bin", but ProcessName is usually without the extension.)
        /// </summary>
        public static ProcessViewModel[] GetAionProcesses()
        {
            var processes = Process.GetProcessesByName("aion.bin");
            return processes.Select(p => new ProcessViewModel(p)).ToArray();
        }
    }
}
