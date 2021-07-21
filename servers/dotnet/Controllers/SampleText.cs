using Microsoft.AspNetCore.Mvc;

namespace WebBench.Controllers
{
    //[ApiController]
    [Route("/sample.txt")]
    public class SampleText : ControllerBase
    {
        private static string text = new string('X', 8192);

        [HttpGet]
        public string Get()
        {
            return text;
        }
    }
}