using System.Diagnostics;

namespace noia.ViewModels
{
    public class ProcessViewModel
    {
        public Process Process { get; }
        public string ProcessName => Process.ProcessName;
        public int Id => Process.Id;
        // New property for the window title:
        public string WindowTitle => Process.MainWindowTitle;

        public ProcessViewModel(Process process)
        {
            Process = process;
        }

        public override string ToString() => WindowTitle;
    }
}
