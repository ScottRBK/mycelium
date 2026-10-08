using System.Collections.Generic;

public class Base { }
public class Payload { }
public class Token { public void Touch() { } }
public interface IRunner { Token? Run(Token? token); }
public enum State { Ready, Done }
public struct Position { public int X; }

public class Worker : Base, IRunner
{
    private Token _token;
    public Token Current { get; set; }

    public Worker(Token token) { _token = token; }
    public Token? Run(Token? token) { return token; }
    public Token Run(int count) { return _token; }
    public List<Payload?> Convert(List<Payload?> items) { return items; }
    public Token Build() { _token.Touch(); return new Token(); }
}
