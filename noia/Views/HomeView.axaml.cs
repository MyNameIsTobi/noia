using System.Diagnostics;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using noia.Helpers;
using noia.ViewModels;

namespace noia.Views
{
    public partial class HomeView : UserControl
    {
        public HomeView()
        {
            InitializeComponent();
            this.AttachedToVisualTree += HomeView_AttachedToVisualTree;
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }

        private void HomeView_AttachedToVisualTree(object? sender, VisualTreeAttachmentEventArgs e)
        {
            LoadProcesses();
        }

        /// <summary>
        /// Loads matching processes (using ProcessHelper). If exactly one process is found,
        /// auto-select it and display module info. If multiple are found, list them in the ComboBox.
        /// If none, display a "not found" message.
        /// </summary>
        private void LoadProcesses()
        {
            // Use the helper to get our wrapped processes.
            ProcessViewModel[] processes = ProcessHelper.GetAionProcesses();
            var processComboBox = this.FindControl<ComboBox>("ProcessComboBox");
            var processInfoText = this.FindControl<TextBlock>("ProcessInfoText");

            if (processes.Length == 1)
            {
                // Exactly one process found: auto-select and display info.
                ProcessViewModel selectedProcess = processes[0];
                processComboBox.ItemsSource = processes;
                processComboBox.SelectedItem = selectedProcess;
                processInfoText.Text = $"Process loaded: {selectedProcess.ProcessName} (ID: {selectedProcess.Id})\n" +
                                       ModuleHelper.GetModuleInfo(selectedProcess.Process, "game.dll");
            }
            else if (processes.Length > 1)
            {
                // Multiple processes: show them in the ComboBox for selection.
                processComboBox.ItemsSource = processes;
                processInfoText.Text = "Multiple matching processes found. Please select one.";

                processComboBox.SelectionChanged -= ProcessComboBox_SelectionChanged;
                processComboBox.SelectionChanged += ProcessComboBox_SelectionChanged;
            }
            else
            {
                // No processes found.
                processComboBox.ItemsSource = null;
                processInfoText.Text = "No matching process found.";
            }
        }

        private void ProcessComboBox_SelectionChanged(object? sender, SelectionChangedEventArgs e)
        {
            var processComboBox = this.FindControl<ComboBox>("ProcessComboBox");
            var processInfoText = this.FindControl<TextBlock>("ProcessInfoText");

            if (processComboBox.SelectedItem is ProcessViewModel selectedProcess)
            {
                processInfoText.Text = $"Process loaded: {selectedProcess.ProcessName} (ID: {selectedProcess.Id})\n" +
                                       ModuleHelper.GetModuleInfo(selectedProcess.Process, "game.dll");
            }
        }

        // Handler for the Reload button.
        private void ReloadButton_Click(object? sender, Avalonia.Interactivity.RoutedEventArgs e)
        {
            LoadProcesses();
        }
    }
}
