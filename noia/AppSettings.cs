using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace noia
{
    public class AppSettings
    {
        public int WindowX { get; set; } = 100;
        public int WindowY { get; set; } = 100;
        public bool StartAtLastPosition { get; set; } = true;
        public bool AlwaysOnTop { get; set; } = false;
    }
}
