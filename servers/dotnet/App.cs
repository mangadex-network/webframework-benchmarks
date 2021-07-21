using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;

namespace WebBench
{
    public class App
    {
        public static void Main(string[] args)
        {
            Host.CreateDefaultBuilder(args)
                .ConfigureWebHostDefaults(app => app.UseKestrel(options => options.AddServerHeader = false).UseStartup<Startup>())
                .Build()
                .Run();
        }
    }

    public class Startup
    {
        public void ConfigureServices(IServiceCollection services)
        {
            services.AddControllers();
        }

        public void Configure(IApplicationBuilder app, IWebHostEnvironment env)
        {
            app.UseRouting();
            app.UseEndpoints(endpoints => endpoints.MapControllers());
            /*
            app.UseEndpoints(endpoints =>
            {
                endpoints.MapGet("/sample.txt", async context => await context.Response.WriteAsync(text));
                endpoints.MapGet("/sample.jpg", async context => {
                    context.Response.ContentType = "image/jpeg"; 
                    await context.Response.SendFileAsync("../../htdocs/sample.jpg");  
                });
            });
            */
        }
    }
}
