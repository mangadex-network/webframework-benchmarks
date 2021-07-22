using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Server.Kestrel.Core;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using System.Net;

namespace WebBench
{
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

    public class App
    {
        /*
        private static void ConfigureLogging(WebHostBuilderContext context, ILoggingBuilder logging)
        {
            // use options from `application.json`, so the file can be removed ...
        }
        */

        private static void ConfigureKestrel(WebHostBuilderContext context, KestrelServerOptions options)
        {
            options.AddServerHeader = false;
            options.Listen(IPAddress.Loopback, 8080);
            options.Listen(IPAddress.Loopback, 44300, listenOptions =>
            {
                listenOptions.UseHttps("../../localhost.pfx");
            });
        }

        private static void ConfigureHost(IWebHostBuilder app)
        {
            //app.ConfigureLogging(ConfigureLogging);
            app.UseKestrel(ConfigureKestrel);
            app.UseStartup<Startup>();
        }

        public static void Main(string[] args)
        {
            Host.CreateDefaultBuilder(args).ConfigureWebHostDefaults(ConfigureHost).Build().Run();
        }
    }
}