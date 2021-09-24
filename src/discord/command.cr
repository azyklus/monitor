# Provides a light framework for commands since our library of choice
# does not provide one for us.
abstract class Command
   abstract def execute
end
