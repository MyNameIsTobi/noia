using System;
using System.Diagnostics;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using Avalonia.Threading;
using noia.Helpers;
using noia.ViewModels;

namespace noia.Views
{
    public partial class HomeView : UserControl
    {
        private DispatcherTimer _memoryUpdateTimer;

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
            // Start a timer to update memory values every 500 milliseconds.
            _memoryUpdateTimer = new DispatcherTimer
            {
                Interval = TimeSpan.FromMilliseconds(250)
            };
            _memoryUpdateTimer.Tick += MemoryUpdateTimer_Tick;
            _memoryUpdateTimer.Start();
        }

        private void MemoryUpdateTimer_Tick(object? sender, EventArgs e)
        {
            // Get the selected process from the ComboBox.
            var processComboBox = this.FindControl<ComboBox>("ProcessComboBox");
            if (processComboBox.SelectedItem is ViewModels.ProcessViewModel selectedProcess)
            {
                try
                {
                    // Use the MemoryReader helper to get all memory values.
                    var values = MemoryReader.ReadMemoryValues(selectedProcess.Process, "game.dll");

                    // Update the HP ProgressBar and percentage.
                    var HPBar = this.FindControl<ProgressBar>("HPBar");
                    var HPPercentageText = this.FindControl<TextBlock>("HPPercentageText");

                    if (values.HPMax > 0)
                    {
                        double hpPercent = (double)values.HP / values.HPMax * 100;
                        HPBar.Value = hpPercent;
                        HPPercentageText.Text = $"{hpPercent:F0}%";
                    }
                    else
                    {
                        HPBar.Value = 0;
                        HPPercentageText.Text = "0%";
                    }

                    // Update the Mana ProgressBar and percentage.
                    var ManaBar = this.FindControl<ProgressBar>("ManaBar");
                    var ManaPercentageText = this.FindControl<TextBlock>("ManaPercentageText");

                    if (values.ManaMax > 0)
                    {
                        double manaPercent = (double)values.Mana / values.ManaMax * 100;
                        ManaBar.Value = manaPercent;
                        ManaPercentageText.Text = $"{manaPercent:F0}%";
                    }
                    else
                    {
                        ManaBar.Value = 0;
                        ManaPercentageText.Text = "0%";
                    }
                }
                catch (Exception ex)
                {
                    var processInfoText = this.FindControl<TextBlock>("ProcessInfoText");
                    processInfoText.Text = $"Error reading memory: {ex.Message}";
                }
            }
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
