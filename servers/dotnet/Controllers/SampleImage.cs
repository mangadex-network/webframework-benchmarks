using Microsoft.AspNetCore.Mvc;

namespace WebBench.Controllers
{
    //[ApiController]
    [Route("/sample.jpg")]
    public class SampleImage : ControllerBase
    {
        [HttpGet]
        public IActionResult Get()
        {
            var stream = System.IO.File.OpenRead("../../htdocs/sample.jpg");
            return new FileStreamResult(stream, "image/jpeg");
        }
    }
}