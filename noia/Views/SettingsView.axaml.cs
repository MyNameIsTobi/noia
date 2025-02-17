using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using noia.Helpers;

namespace noia.Views
{
    public partial class SettingsView : UserControl
    {
        public SettingsView()
        {
            InitializeComponent();

            var startToggle = this.FindControl<ToggleSwitch>("StartAtLastPositionToggle");
            var alwaysOnTopToggle = this.FindControl<ToggleSwitch>("AlwaysOnTopToggle");

            // Initialize toggles from settings.
            startToggle.IsChecked = SettingsManager.Settings.StartAtLastPosition;
            alwaysOnTopToggle.IsChecked = SettingsManager.Settings.AlwaysOnTop;

            // Subscribe to PropertyChanged events.
            startToggle.PropertyChanged += (sender, e) =>
            {
                if (e.Property == ToggleSwitch.IsCheckedProperty)
                {
                    bool? isChecked = startToggle.IsChecked;
                    SettingsManager.Settings.StartAtLastPosition = isChecked ?? false;
                    SettingsManager.SaveSettings();
                }
            };

            alwaysOnTopToggle.PropertyChanged += (sender, e) =>
            {
                if (e.Property == ToggleSwitch.IsCheckedProperty)
                {
                    bool? isChecked = alwaysOnTopToggle.IsChecked;
                    SettingsManager.Settings.AlwaysOnTop = isChecked ?? false;
                    SettingsManager.SaveSettings();

                    if (App.Current?.ApplicationLifetime is Avalonia.Controls.ApplicationLifetimes.IClassicDesktopStyleApplicationLifetime desktop)
                    {
                        desktop.MainWindow.Topmost = isChecked ?? false;
                    }
                }
            };
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}
