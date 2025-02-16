using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using Avalonia.Media;
using noia.Views;

namespace noia
{
    public partial class MainWindow : Window
    {
        private Button _homeButton;
        private Button _skillsButton;
        private Button _settingsButton;

        public MainWindow()
        {
            SettingsManager.LoadSettings();
            InitializeComponent();

            this.Opened += (sender, e) =>
            {
                this.Position = new PixelPoint(SettingsManager.Settings.WindowX, SettingsManager.Settings.WindowY);
            };

            this.Closing += (sender, e) =>
            {
                var pos = this.Position;
                SettingsManager.Settings.WindowX = pos.X;
                SettingsManager.Settings.WindowY = pos.Y;
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
