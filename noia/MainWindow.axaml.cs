using System;
using System.Diagnostics;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using Avalonia.Media;
using noia.Helpers;
using noia.Views;

namespace noia
{
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
#if DEBUG
            this.AttachDevTools();
#endif

            SettingsManager.LoadSettings();

            // If "Start at last position" is enabled, restore the saved window position.
            if (SettingsManager.Settings.StartAtLastPosition)
            {
                this.Opened += (sender, e) =>
                {
                    this.Position = new PixelPoint(SettingsManager.Settings.WindowX, SettingsManager.Settings.WindowY);
                };
            }
            else
            {
                // Otherwise, use a default startup location. For example, center the window.
                this.WindowStartupLocation = WindowStartupLocation.CenterScreen;
            }

            // Apply "Always on top" setting.
            this.Topmost = SettingsManager.Settings.AlwaysOnTop;

            // Save window position on closing, if "Start at last position" is enabled.
            this.Closing += (sender, e) =>
            {
                if (SettingsManager.Settings.StartAtLastPosition)
                {
                    var pos = this.Position;
                    SettingsManager.Settings.WindowX = pos.X;
                    SettingsManager.Settings.WindowY = pos.Y;
                }
                SettingsManager.SaveSettings();
            };

            Avalonia.Threading.Dispatcher.UIThread.Post(() =>
            {
                var mainContent = this.FindControl<ContentControl>("MainContent");
                mainContent.Content = new HomeView();
            });
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }

        private void MinimizeButton_Click(object sender, Avalonia.Interactivity.RoutedEventArgs e)
        {
            this.WindowState = WindowState.Minimized;
        }

        private void CloseButton_Click(object sender, Avalonia.Interactivity.RoutedEventArgs e)
        {
            this.Close();
        }


        private void HomeButton_Click(object sender, Avalonia.Interactivity.RoutedEventArgs e)
        {
            var mainContent = this.FindControl<ContentControl>("MainContent");
            if (mainContent != null)
            {
                mainContent.Content = new HomeView();
            }
            else
            {
                // Optionally log or handle the error if the control isn't found.
            }
        }

        private void SkillsButton_Click(object sender, Avalonia.Interactivity.RoutedEventArgs e)
        {
            var mainContent = this.FindControl<ContentControl>("MainContent");
            if (mainContent != null)
            {
                mainContent.Content = new SkillsView();
            }
            else
            {
                // Optionally log or handle the error if the control isn't found.
            }
        }

        private void SettingsButton_Click(object sender, Avalonia.Interactivity.RoutedEventArgs e)
        {
            var mainContent = this.FindControl<ContentControl>("MainContent");
            if (mainContent != null)
            {
                mainContent.Content = new SettingsView();
            }
            else
            {
                // Optionally log or handle the error if the control isn't found.
            }
        }
    }
}
