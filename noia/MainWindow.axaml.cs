using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
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
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
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

    }
}
